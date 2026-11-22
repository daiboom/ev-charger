use std::time::Duration;

use thiserror::Error;
use tracing::info;

use crate::domain::{ChargeSpeed, ChargeTarget, PaymentMethod, Selection};
use crate::port::{
    ChargerPort, ChargerSnapshot, ConnectionState, ContactorCommand, PaymentPort, PaymentStatus,
};

/// Placeholder tariff and power caps for the desktop simulator.
/// These are not a charger nameplate.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub tariff_per_kwh: f64,
    pub currency: String,
    pub fast_kw: f32,
    pub normal_kw: f32,
    pub slow_kw: f32,
    pub payment_approval_delay: Duration,
    pub full_soc_percent: f32,
}

impl SessionConfig {
    pub fn simulator_defaults() -> Self {
        Self {
            tariff_per_kwh: 300.0,
            currency: "KRW".to_string(),
            fast_kw: 50.0,
            normal_kw: 7.0,
            slow_kw: 3.0,
            payment_approval_delay: Duration::from_secs(1),
            full_soc_percent: 100.0,
        }
    }

    pub fn power_limit_kw(&self, speed: ChargeSpeed) -> f32 {
        match speed {
            ChargeSpeed::Fast => self.fast_kw,
            ChargeSpeed::Normal => self.normal_kw,
            ChargeSpeed::Slow => self.slow_kw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    UserStop,
    TargetReached,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    Standby,
    WaitingForConnection,
    AwaitingPayment,
    Charging,
    Stopping {
        reason: StopReason,
    },
    Complete,
    Fault {
        message: String,
        delivered_kwh: f64,
        cost: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChargeSummary {
    pub selection: Selection,
    pub payment_method: PaymentMethod,
    pub transaction_id: String,
    pub delivered_kwh: f64,
    pub duration: Duration,
    pub cost: f64,
    pub currency: String,
    pub soc_percent: Option<f32>,
    pub stop_reason: StopReason,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiveCharge {
    pub delivered_kwh: f64,
    pub power_kw: f32,
    pub voltage_v: f32,
    pub current_a: f32,
    pub soc_percent: Option<f32>,
    pub elapsed: Duration,
    pub cost: f64,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SessionError {
    #[error("a target can be chosen only from standby")]
    NotStandby,
    #[error("payment can be started only while waiting for payment")]
    NotAwaitingPayment,
    #[error("energy must be positive, and state of charge must be within 0 to 100")]
    InvalidTarget,
}

/// Owns the contactor command for one connector.
///
/// A new session always starts in standby with the contactor open. Nothing in
/// this type reloads a previous charging phase.
pub struct ChargeSession<C, P> {
    config: SessionConfig,
    phase: Phase,
    charger: C,
    payment: P,
    selection: Option<Selection>,
    /// Set when the contactor has been closed for this visit. The simulator
    /// meter keeps its total, so a later fault must not bill that history.
    output_started: bool,
    start_energy_kwh: f64,
    elapsed: Duration,
    summary: Option<ChargeSummary>,
}

impl<C, P> ChargeSession<C, P>
where
    C: ChargerPort,
    P: PaymentPort,
{
    pub fn new(config: SessionConfig, mut charger: C, payment: P) -> Self {
        let _ = charger.set_contactor(ContactorCommand::Open);
        charger.set_power_limit_kw(0.0);
        Self {
            config,
            phase: Phase::Standby,
            charger,
            payment,
            selection: None,
            output_started: false,
            start_energy_kwh: 0.0,
            elapsed: Duration::ZERO,
            summary: None,
        }
    }

    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }

    pub fn summary(&self) -> Option<&ChargeSummary> {
        self.summary.as_ref()
    }

    pub fn payment_status(&self) -> PaymentStatus {
        self.payment.status()
    }

    pub fn snapshot(&self) -> ChargerSnapshot {
        self.charger.snapshot()
    }

    pub fn charger(&self) -> &C {
        &self.charger
    }

    pub fn charger_mut(&mut self) -> &mut C {
        &mut self.charger
    }

    pub fn select(&mut self, target: ChargeTarget, speed: ChargeSpeed) -> Result<(), SessionError> {
        if !matches!(self.phase, Phase::Standby) {
            return Err(SessionError::NotStandby);
        }
        validate_target(target)?;
        self.selection = Some(Selection { target, speed });
        self.summary = None;
        self.elapsed = Duration::ZERO;
        self.set_phase(Phase::WaitingForConnection);
        Ok(())
    }

    pub fn choose_payment(&mut self, method: PaymentMethod) -> Result<(), SessionError> {
        if !matches!(self.phase, Phase::AwaitingPayment) {
            return Err(SessionError::NotAwaitingPayment);
        }
        info!(method = method.label(), "payment requested");
        self.payment.begin(method);
        Ok(())
    }

    /// Waiting and payment return to standby.
    /// Charging opens the contactor and ends as a normal partial session.
    pub fn request_stop(&mut self) {
        match self.phase {
            Phase::WaitingForConnection | Phase::AwaitingPayment => {
                self.payment.reset();
                self.selection = None;
                self.open_output();
                self.set_phase(Phase::Standby);
            }
            Phase::Charging => self.begin_stop(StopReason::UserStop),
            Phase::Standby | Phase::Stopping { .. } | Phase::Complete | Phase::Fault { .. } => {}
        }
    }

    pub fn acknowledge(&mut self) {
        if !matches!(self.phase, Phase::Complete | Phase::Fault { .. }) {
            return;
        }
        self.payment.reset();
        self.selection = None;
        self.summary = None;
        self.elapsed = Duration::ZERO;
        self.output_started = false;
        self.start_energy_kwh = 0.0;
        self.open_output();
        self.set_phase(Phase::Standby);
    }

    pub fn estimate_cost(&self) -> Option<f64> {
        let selection = self.selection?;
        let snap = self.charger.snapshot();
        let kwh = match selection.target {
            ChargeTarget::Energy { kwh } => f64::from(kwh),
            ChargeTarget::Soc { percent } => energy_for_soc(
                snap.vehicle_soc_percent?,
                percent,
                snap.battery_capacity_kwh?,
            )?,
            ChargeTarget::Full => energy_for_soc(
                snap.vehicle_soc_percent?,
                self.config.full_soc_percent,
                snap.battery_capacity_kwh?,
            )?,
        };
        Some(self.cost_of(kwh))
    }

    pub fn live_charge(&self) -> Option<LiveCharge> {
        if !matches!(self.phase, Phase::Charging | Phase::Stopping { .. }) {
            return None;
        }
        let snap = self.charger.snapshot();
        let delivered = self.delivered_kwh();
        Some(LiveCharge {
            delivered_kwh: delivered,
            power_kw: snap.meter.power_kw,
            voltage_v: snap.meter.voltage_v,
            current_a: snap.meter.current_a,
            soc_percent: snap.vehicle_soc_percent,
            elapsed: self.elapsed,
            cost: self.cost_of(delivered),
            currency: self.config.currency.clone(),
        })
    }

    pub fn tick(&mut self, dt: Duration) {
        self.charger.tick(dt);
        self.payment.tick(dt);
        match self.phase.clone() {
            Phase::WaitingForConnection => self.poll_connection(),
            Phase::AwaitingPayment => self.poll_payment(),
            Phase::Charging => self.poll_charging(dt),
            Phase::Stopping { reason } => self.poll_stopping(reason),
            Phase::Standby | Phase::Complete | Phase::Fault { .. } => {}
        }
    }

    fn poll_connection(&mut self) {
        let snap = self.charger.snapshot();
        if let Some(fault) = snap.fault {
            self.enter_fault(fault.message());
            return;
        }
        if snap.connection == ConnectionState::Ready {
            self.set_phase(Phase::AwaitingPayment);
        }
    }

    fn poll_payment(&mut self) {
        let snap = self.charger.snapshot();
        if let Some(fault) = snap.fault {
            self.enter_fault(fault.message());
            return;
        }
        if snap.connection != ConnectionState::Ready {
            self.payment.reset();
            self.set_phase(Phase::WaitingForConnection);
            return;
        }
        if let PaymentStatus::Approved { .. } = self.payment.status() {
            self.start_charging();
        }
    }

    fn start_charging(&mut self) {
        let Some(selection) = self.selection else {
            self.enter_fault("Charging selection is missing");
            return;
        };
        let snap = self.charger.snapshot();
        if snap.fault.is_some() || snap.connection != ConnectionState::Ready {
            self.enter_fault("Charger is not ready to start");
            return;
        }
        if !matches!(self.payment.status(), PaymentStatus::Approved { .. }) {
            self.enter_fault("Payment is not approved");
            return;
        }

        let limit = self.config.power_limit_kw(selection.speed);
        self.charger.set_power_limit_kw(limit);
        // Close only after a ready vehicle and an approved payment.
        if let Err(error) = self.charger.set_contactor(ContactorCommand::Closed) {
            self.open_output();
            self.enter_fault(error.to_string());
            return;
        }
        self.start_energy_kwh = self.charger.snapshot().meter.energy_kwh;
        self.output_started = true;
        self.elapsed = Duration::ZERO;
        info!(power_limit_kw = limit, "contactor closed");
        self.set_phase(Phase::Charging);
    }

    fn poll_charging(&mut self, dt: Duration) {
        self.elapsed = self.elapsed.saturating_add(dt);
        if self.safety_stop() {
            return;
        }
        let snap = self.charger.snapshot();
        if self.target_reached(&snap) {
            self.begin_stop(StopReason::TargetReached);
        }
    }

    fn poll_stopping(&mut self, reason: StopReason) {
        if self.safety_stop() {
            return;
        }
        self.open_output();
        let snap = self.charger.snapshot();
        if snap.contactor == ContactorCommand::Open && snap.meter.power_kw <= 0.05 {
            self.finish(reason);
        }
    }

    /// Faults and a lost vehicle open the output and leave the charging flow.
    fn safety_stop(&mut self) -> bool {
        let snap = self.charger.snapshot();
        if let Some(fault) = snap.fault {
            self.enter_fault(fault.message());
            return true;
        }
        if snap.connection != ConnectionState::Ready {
            self.enter_fault("Connector is not ready");
            return true;
        }
        false
    }

    fn begin_stop(&mut self, reason: StopReason) {
        self.open_output();
        self.set_phase(Phase::Stopping { reason });
        self.poll_stopping(reason);
    }

    fn finish(&mut self, reason: StopReason) {
        let Some(selection) = self.selection else {
            self.enter_fault("Charging selection is missing");
            return;
        };
        let (payment_method, transaction_id) = match self.payment.status() {
            PaymentStatus::Approved {
                method,
                transaction_id,
            } => (method, transaction_id),
            _ => {
                self.enter_fault("Payment approval was lost before settlement");
                return;
            }
        };
        let snap = self.charger.snapshot();
        let delivered = self.delivered_kwh();
        self.summary = Some(ChargeSummary {
            selection,
            payment_method,
            transaction_id,
            delivered_kwh: delivered,
            duration: self.elapsed,
            cost: self.cost_of(delivered),
            currency: self.config.currency.clone(),
            soc_percent: snap.vehicle_soc_percent,
            stop_reason: reason,
        });
        self.open_output();
        self.set_phase(Phase::Complete);
    }

    fn enter_fault(&mut self, message: impl Into<String>) {
        if matches!(self.phase, Phase::Fault { .. }) {
            return;
        }
        let delivered = self.delivered_kwh();
        let cost = self.cost_of(delivered);
        self.open_output();
        self.payment.reset();
        let message = message.into();
        info!(%message, delivered_kwh = delivered, "session fault");
        self.phase = Phase::Fault {
            message,
            delivered_kwh: delivered,
            cost,
        };
    }

    fn open_output(&mut self) {
        let _ = self.charger.set_contactor(ContactorCommand::Open);
        self.charger.set_power_limit_kw(0.0);
    }

    fn delivered_kwh(&self) -> f64 {
        if !self.output_started {
            return 0.0;
        }
        (self.charger.snapshot().meter.energy_kwh - self.start_energy_kwh).max(0.0)
    }

    fn cost_of(&self, kwh: f64) -> f64 {
        (kwh.max(0.0)) * self.config.tariff_per_kwh
    }

    fn target_reached(&self, snap: &ChargerSnapshot) -> bool {
        let Some(selection) = self.selection else {
            return true;
        };
        match selection.target {
            ChargeTarget::Energy { kwh } => self.delivered_kwh() + 1e-4 >= f64::from(kwh),
            ChargeTarget::Soc { percent } => {
                snap.vehicle_soc_percent.unwrap_or(0.0) + 0.02 >= percent
            }
            ChargeTarget::Full => {
                snap.vehicle_soc_percent.unwrap_or(0.0) + 0.02 >= self.config.full_soc_percent
            }
        }
    }

    fn set_phase(&mut self, phase: Phase) {
        if self.phase != phase {
            info!(?phase, "session phase");
            self.phase = phase;
        }
    }
}

fn validate_target(target: ChargeTarget) -> Result<(), SessionError> {
    let valid = match target {
        ChargeTarget::Energy { kwh } => kwh.is_finite() && kwh > 0.0,
        ChargeTarget::Soc { percent } => percent.is_finite() && percent > 0.0 && percent <= 100.0,
        ChargeTarget::Full => true,
    };
    if valid {
        Ok(())
    } else {
        Err(SessionError::InvalidTarget)
    }
}

fn energy_for_soc(current: f32, target: f32, capacity_kwh: f32) -> Option<f64> {
    if !current.is_finite()
        || !target.is_finite()
        || !capacity_kwh.is_finite()
        || capacity_kwh <= 0.0
    {
        return None;
    }
    Some(f64::from((target - current).max(0.0) / 100.0) * f64::from(capacity_kwh))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::ChargerFault;
    use crate::sim::{FakePayment, Simulator};

    fn config() -> SessionConfig {
        SessionConfig {
            tariff_per_kwh: 1000.0,
            currency: "KRW".to_string(),
            fast_kw: 3600.0,
            normal_kw: 1800.0,
            slow_kw: 900.0,
            payment_approval_delay: Duration::ZERO,
            full_soc_percent: 100.0,
        }
    }

    fn session() -> ChargeSession<Simulator, FakePayment> {
        let mut sim = Simulator::new();
        sim.set_time_scale(1.0);
        sim.set_capacity_kwh(100.0);
        sim.set_soc_percent(20.0);
        ChargeSession::new(config(), sim, FakePayment::immediate())
    }

    fn arm(session: &mut ChargeSession<Simulator, FakePayment>) {
        session.charger_mut().plug_in();
        session.charger_mut().make_ready();
        session.tick(Duration::from_millis(1));
    }

    fn start_energy(session: &mut ChargeSession<Simulator, FakePayment>, kwh: f32) {
        session
            .select(ChargeTarget::Energy { kwh }, ChargeSpeed::Fast)
            .unwrap();
        arm(session);
        assert!(matches!(session.phase(), Phase::AwaitingPayment));
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        session.choose_payment(PaymentMethod::CreditCard).unwrap();
        session.tick(Duration::ZERO);
        assert!(matches!(session.phase(), Phase::Charging));
        assert_eq!(session.snapshot().contactor, ContactorCommand::Closed);
    }

    #[test]
    fn a_new_session_is_standby_with_the_contactor_open() {
        let session = session();
        assert_eq!(session.phase(), &Phase::Standby);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        assert!(session.summary().is_none());
    }

    #[test]
    fn the_contactor_stays_open_until_payment_is_approved() {
        let mut session = session();
        session
            .select(ChargeTarget::Energy { kwh: 1.0 }, ChargeSpeed::Fast)
            .unwrap();
        session.tick(Duration::from_secs(5));
        assert_eq!(session.phase(), &Phase::WaitingForConnection);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);

        session.charger_mut().plug_in();
        session.tick(Duration::from_secs(5));
        assert_eq!(session.phase(), &Phase::WaitingForConnection);
        assert_eq!(session.snapshot().connection, ConnectionState::Connected);

        session.charger_mut().make_ready();
        session.tick(Duration::from_millis(1));
        assert_eq!(session.phase(), &Phase::AwaitingPayment);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        assert_eq!(session.choose_payment(PaymentMethod::Rfid), Ok(()));
    }

    #[test]
    fn payment_cannot_start_from_standby() {
        let mut session = session();
        assert_eq!(
            session.choose_payment(PaymentMethod::CreditCard),
            Err(SessionError::NotAwaitingPayment)
        );
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
    }

    #[test]
    fn a_delayed_approval_does_not_close_early_and_cancel_drops_it() {
        let mut sim = Simulator::new();
        sim.set_time_scale(1.0);
        let mut session =
            ChargeSession::new(config(), sim, FakePayment::new(Duration::from_secs(2)));
        session
            .select(ChargeTarget::Energy { kwh: 1.0 }, ChargeSpeed::Normal)
            .unwrap();
        arm(&mut session);
        session.choose_payment(PaymentMethod::MobileApp).unwrap();
        session.tick(Duration::from_secs(1));
        assert!(matches!(
            session.payment_status(),
            PaymentStatus::Pending { .. }
        ));
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        session.request_stop();
        session.tick(Duration::from_secs(5));
        assert_eq!(session.phase(), &Phase::Standby);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        assert!(matches!(session.payment_status(), PaymentStatus::Idle));
    }

    #[test]
    fn a_declined_payment_does_not_close_the_contactor() {
        let mut sim = Simulator::new();
        sim.set_time_scale(1.0);
        let mut payment = FakePayment::immediate();
        payment.decline_next();
        let mut session = ChargeSession::new(config(), sim, payment);
        session
            .select(ChargeTarget::Full, ChargeSpeed::Slow)
            .unwrap();
        arm(&mut session);
        session.choose_payment(PaymentMethod::Membership).unwrap();
        session.tick(Duration::from_secs(1));
        assert!(matches!(
            session.payment_status(),
            PaymentStatus::Declined { .. }
        ));
        assert_eq!(session.phase(), &Phase::AwaitingPayment);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
    }

    #[test]
    fn energy_target_stops_and_bills_delivered_kwh() {
        let mut session = session();
        start_energy(&mut session, 1.0);
        session.tick(Duration::from_secs(1));
        let summary = session.summary().expect("completed summary");
        assert_eq!(session.phase(), &Phase::Complete);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        assert!((summary.delivered_kwh - 1.0).abs() < 1e-3);
        assert!((summary.cost - 1000.0).abs() < 1.0);
        assert_eq!(summary.currency, "KRW");
        assert_eq!(summary.stop_reason, StopReason::TargetReached);
        assert_eq!(summary.payment_method, PaymentMethod::CreditCard);
        assert!(summary.transaction_id.starts_with("SIM-"));
        assert_eq!(session.snapshot().meter.power_kw, 0.0);
    }

    #[test]
    fn state_of_charge_target_uses_the_vehicle_report() {
        let mut session = session();
        session.charger_mut().set_capacity_kwh(10.0);
        session.charger_mut().set_soc_percent(50.0);
        session
            .select(ChargeTarget::Soc { percent: 60.0 }, ChargeSpeed::Fast)
            .unwrap();
        arm(&mut session);
        session.choose_payment(PaymentMethod::Rfid).unwrap();
        session.tick(Duration::ZERO);
        session.tick(Duration::from_secs(1));
        let summary = session.summary().unwrap();
        assert_eq!(summary.stop_reason, StopReason::TargetReached);
        assert!((summary.soc_percent.unwrap() - 60.0).abs() < 0.05);
        assert!((summary.delivered_kwh - 1.0).abs() < 1e-2);
    }

    #[test]
    fn full_charge_stops_at_the_configured_full_level() {
        let mut session = session();
        session.charger_mut().set_capacity_kwh(10.0);
        session.charger_mut().set_soc_percent(80.0);
        session
            .select(ChargeTarget::Full, ChargeSpeed::Fast)
            .unwrap();
        arm(&mut session);
        session.choose_payment(PaymentMethod::CreditCard).unwrap();
        session.tick(Duration::ZERO);
        session.tick(Duration::from_secs(2));
        let summary = session.summary().unwrap();
        assert!(summary.soc_percent.unwrap() >= 99.9);
        assert_eq!(summary.stop_reason, StopReason::TargetReached);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
    }

    #[test]
    fn user_stop_opens_the_contactor_and_settles_partial_energy() {
        let mut session = session();
        start_energy(&mut session, 5.0);
        session.tick(Duration::from_millis(500));
        assert!(matches!(session.phase(), Phase::Charging));
        session.request_stop();
        let summary = session.summary().unwrap();
        assert_eq!(summary.stop_reason, StopReason::UserStop);
        assert!((summary.delivered_kwh - 0.5).abs() < 1e-2);
        assert!((summary.cost - 500.0).abs() < 10.0);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
    }

    #[test]
    fn a_fault_during_charging_opens_the_output() {
        let mut session = session();
        start_energy(&mut session, 5.0);
        session.tick(Duration::from_millis(200));
        session.charger_mut().inject_fault();
        session.tick(Duration::from_millis(1));
        match session.phase() {
            Phase::Fault {
                message,
                delivered_kwh,
                ..
            } => {
                assert_eq!(message.as_str(), ChargerFault::Internal.message());
                assert!(*delivered_kwh > 0.0);
            }
            other => panic!("expected fault, got {other:?}"),
        }
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
    }

    #[test]
    fn unplugging_during_charging_is_a_fault_and_opens_the_contactor() {
        let mut session = session();
        start_energy(&mut session, 5.0);
        session.tick(Duration::from_millis(100));
        session.charger_mut().unplug();
        session.tick(Duration::from_millis(1));
        match session.phase() {
            Phase::Fault { message, .. } => {
                assert_eq!(
                    message.as_str(),
                    ChargerFault::DisconnectedUnderLoad.message()
                );
            }
            other => panic!("expected fault, got {other:?}"),
        }
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        assert_eq!(session.snapshot().meter.power_kw, 0.0);
    }

    #[test]
    fn acknowledge_returns_to_standby_and_does_not_resume_charging() {
        let mut session = session();
        start_energy(&mut session, 1.0);
        session.tick(Duration::from_secs(1));
        assert_eq!(session.phase(), &Phase::Complete);
        session.acknowledge();
        session.tick(Duration::from_secs(3));
        assert_eq!(session.phase(), &Phase::Standby);
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
        assert!(session.summary().is_none());
        assert!(session.live_charge().is_none());
    }

    #[test]
    fn speed_selects_the_configured_power_cap() {
        let mut session = session();
        session
            .select(ChargeTarget::Energy { kwh: 2.0 }, ChargeSpeed::Slow)
            .unwrap();
        arm(&mut session);
        session.choose_payment(PaymentMethod::CreditCard).unwrap();
        session.tick(Duration::from_millis(1));
        session.tick(Duration::from_millis(1));
        let live = session.live_charge().unwrap();
        assert!((live.power_kw - 900.0).abs() < 1.0);
    }

    #[test]
    fn estimate_uses_requested_energy_and_rejects_an_empty_target() {
        let mut session = session();
        assert_eq!(
            session.select(ChargeTarget::Energy { kwh: 0.0 }, ChargeSpeed::Fast),
            Err(SessionError::InvalidTarget)
        );
        assert_eq!(
            session.select(ChargeTarget::Soc { percent: 140.0 }, ChargeSpeed::Fast),
            Err(SessionError::InvalidTarget)
        );
        session
            .select(ChargeTarget::Energy { kwh: 2.5 }, ChargeSpeed::Normal)
            .unwrap();
        let estimate = session.estimate_cost().unwrap();
        assert!((estimate - 2500.0).abs() < 1e-6);
    }

    #[test]
    fn a_ready_fault_blocks_the_contactor_after_approval() {
        let mut session = session();
        session
            .select(ChargeTarget::Energy { kwh: 1.0 }, ChargeSpeed::Fast)
            .unwrap();
        arm(&mut session);
        session.charger_mut().inject_fault();
        session.choose_payment(PaymentMethod::CreditCard).unwrap();
        session.tick(Duration::ZERO);
        assert!(matches!(session.phase(), Phase::Fault { .. }));
        assert_eq!(session.snapshot().contactor, ContactorCommand::Open);
    }
}
