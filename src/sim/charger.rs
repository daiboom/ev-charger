use std::time::Duration;

use crate::port::{
    ChargerFault, ChargerPort, ChargerSnapshot, ConnectionState, ContactorCommand, Meter, PortError,
};

/// Wall-clock multiplier for the demo. One real second advances one simulated minute.
pub const DEMO_TIME_SCALE: f64 = 60.0;

/// In-memory vehicle and contactor. This is the only charger adapter shipped here.
#[derive(Debug, Clone)]
pub struct Simulator {
    connector_id: u8,
    connection: ConnectionState,
    contactor: ContactorCommand,
    power_limit_kw: f32,
    applied_power_kw: f32,
    energy_kwh: f64,
    voltage_v: f32,
    soc_percent: f32,
    capacity_kwh: f32,
    fault: Option<ChargerFault>,
    /// Multiplies elapsed time inside [`ChargerPort::tick`]. Tests use `1.0`.
    time_scale: f64,
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            connector_id: 1,
            connection: ConnectionState::Disconnected,
            contactor: ContactorCommand::Open,
            power_limit_kw: 0.0,
            applied_power_kw: 0.0,
            energy_kwh: 0.0,
            voltage_v: 400.0,
            soc_percent: 40.0,
            capacity_kwh: 60.0,
            fault: None,
            time_scale: DEMO_TIME_SCALE,
        }
    }

    pub fn time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale.max(0.0);
    }

    pub fn set_soc_percent(&mut self, soc: f32) {
        self.soc_percent = soc.clamp(0.0, 100.0);
    }

    pub fn set_capacity_kwh(&mut self, capacity_kwh: f32) {
        self.capacity_kwh = capacity_kwh.max(0.1);
    }

    pub fn plug_in(&mut self) {
        if self.fault.is_some() {
            return;
        }
        if self.connection == ConnectionState::Disconnected {
            self.connection = ConnectionState::Connected;
        }
    }

    pub fn make_ready(&mut self) {
        if self.fault.is_some() {
            return;
        }
        if matches!(
            self.connection,
            ConnectionState::Connected | ConnectionState::Ready
        ) {
            self.connection = ConnectionState::Ready;
        }
    }

    pub fn unplug(&mut self) {
        if self.contactor == ContactorCommand::Closed {
            self.force_open(Some(ChargerFault::DisconnectedUnderLoad));
        }
        self.connection = ConnectionState::Disconnected;
    }

    pub fn inject_fault(&mut self) {
        self.force_open(Some(ChargerFault::Internal));
    }

    pub fn clear_fault(&mut self) {
        self.fault = None;
    }

    fn force_open(&mut self, fault: Option<ChargerFault>) {
        self.contactor = ContactorCommand::Open;
        self.applied_power_kw = 0.0;
        if let Some(fault) = fault {
            self.fault = Some(fault);
        }
    }
}

impl ChargerPort for Simulator {
    fn snapshot(&self) -> ChargerSnapshot {
        let current_a = if self.voltage_v > 0.0 {
            self.applied_power_kw * 1000.0 / self.voltage_v
        } else {
            0.0
        };
        ChargerSnapshot {
            connector_id: self.connector_id,
            connection: self.connection,
            contactor: self.contactor,
            meter: Meter {
                energy_kwh: self.energy_kwh,
                power_kw: self.applied_power_kw,
                voltage_v: self.voltage_v,
                current_a,
            },
            vehicle_soc_percent: Some(self.soc_percent),
            battery_capacity_kwh: Some(self.capacity_kwh),
            fault: self.fault,
        }
    }

    fn set_contactor(&mut self, command: ContactorCommand) -> Result<(), PortError> {
        match command {
            ContactorCommand::Open => {
                self.contactor = ContactorCommand::Open;
                self.applied_power_kw = 0.0;
                Ok(())
            }
            ContactorCommand::Closed => {
                if self.fault.is_some() {
                    return Err(PortError::FaultActive);
                }
                if self.connection != ConnectionState::Ready {
                    return Err(PortError::NotReady);
                }
                self.contactor = ContactorCommand::Closed;
                Ok(())
            }
        }
    }

    fn set_power_limit_kw(&mut self, kw: f32) {
        self.power_limit_kw = kw.max(0.0);
    }

    fn tick(&mut self, dt: Duration) {
        let can_deliver = self.contactor == ContactorCommand::Closed
            && self.connection == ConnectionState::Ready
            && self.fault.is_none()
            && self.soc_percent < 100.0;
        if !can_deliver {
            self.applied_power_kw = 0.0;
            return;
        }

        let remaining_kwh =
            f64::from((100.0 - self.soc_percent).max(0.0) / 100.0) * f64::from(self.capacity_kwh);
        let hours = dt.as_secs_f64() / 3600.0 * self.time_scale;
        let offered = f64::from(self.power_limit_kw) * hours;
        let delta = offered.min(remaining_kwh).max(0.0);
        self.energy_kwh += delta;
        if self.capacity_kwh > 0.0 {
            self.soc_percent =
                (self.soc_percent + (delta as f32 / self.capacity_kwh) * 100.0).clamp(0.0, 100.0);
        }
        self.applied_power_kw = if hours > 0.0 {
            (delta / hours) as f32
        } else {
            0.0
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_is_rejected_until_the_vehicle_is_ready() {
        let mut sim = Simulator::new();
        assert_eq!(
            sim.set_contactor(ContactorCommand::Closed),
            Err(PortError::NotReady)
        );
        sim.plug_in();
        assert_eq!(
            sim.set_contactor(ContactorCommand::Closed),
            Err(PortError::NotReady)
        );
        sim.make_ready();
        assert_eq!(sim.set_contactor(ContactorCommand::Closed), Ok(()));
        assert_eq!(sim.snapshot().contactor, ContactorCommand::Closed);
    }

    #[test]
    fn close_is_rejected_while_a_fault_is_latched() {
        let mut sim = Simulator::new();
        sim.plug_in();
        sim.make_ready();
        sim.inject_fault();
        assert_eq!(
            sim.set_contactor(ContactorCommand::Closed),
            Err(PortError::FaultActive)
        );
        sim.clear_fault();
        sim.plug_in();
        sim.make_ready();
        assert_eq!(sim.set_contactor(ContactorCommand::Closed), Ok(()));
    }

    #[test]
    fn unplug_under_load_opens_the_contactor_and_latches_a_fault() {
        let mut sim = Simulator::new();
        sim.plug_in();
        sim.make_ready();
        sim.set_contactor(ContactorCommand::Closed).unwrap();
        sim.unplug();
        let snap = sim.snapshot();
        assert_eq!(snap.contactor, ContactorCommand::Open);
        assert_eq!(snap.connection, ConnectionState::Disconnected);
        assert_eq!(snap.fault, Some(ChargerFault::DisconnectedUnderLoad));
        assert_eq!(snap.meter.power_kw, 0.0);
    }
}
