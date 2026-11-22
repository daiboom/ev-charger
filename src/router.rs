use std::time::{Duration, Instant};

use eframe::egui;
use tracing::info;

use crate::domain::{ChargeSession, PaymentMethod, Phase, SessionConfig};
use crate::layout::app_container::calculate_scale;
use crate::layout::top_bar::show_top_bar;
use crate::port::ContactorCommand;
use crate::screen::charging_screen::{self, ChargingAction};
use crate::screen::complete_screen;
use crate::screen::connect_screen;
use crate::screen::fault_screen::{self, FaultAction};
use crate::screen::payment_screen::{self, PaymentAction, PaymentView};
use crate::screen::select_amount_screen::{self, AmountDraft, SelectAction};
use crate::screen::splash_screen::SplashScreen;
use crate::screen::standby_screen;
use crate::sim::{FakePayment, Simulator};

pub struct Router {
    session: ChargeSession<Simulator, FakePayment>,
    splash: SplashScreen,
    show_splash: bool,
    last_tick: Instant,
    draft: Option<AmountDraft>,
    selected_method: Option<PaymentMethod>,
}

impl Router {
    pub fn new() -> Self {
        let config = SessionConfig::simulator_defaults();
        let payment = FakePayment::new(config.payment_approval_delay);
        let session = ChargeSession::new(config, Simulator::new(), payment);
        Self {
            session,
            splash: load_splash(),
            show_splash: true,
            last_tick: Instant::now(),
            draft: None,
            selected_method: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_splash {
            self.splash.show(ctx);
            if self.splash.is_finished() {
                self.show_splash = false;
                self.last_tick = Instant::now();
                info!("splash finished");
            }
            ctx.request_repaint_after(Duration::from_millis(16));
            return;
        }

        let mut dt = self.last_tick.elapsed();
        self.last_tick = Instant::now();
        if dt > Duration::from_millis(250) {
            dt = Duration::from_millis(250);
        }
        self.session.tick(dt);

        if !matches!(self.session.phase(), Phase::AwaitingPayment) {
            self.selected_method = None;
        }

        let scale = calculate_scale(ctx);
        show_top_bar(ctx, scale, &self.status_line());

        if self.draft.is_some() && matches!(self.session.phase(), Phase::Standby) {
            self.show_draft(ctx);
        } else {
            self.draft = None;
            self.show_phase(ctx);
        }

        ctx.request_repaint_after(self.repaint_after());
    }

    fn show_draft(&mut self, ctx: &egui::Context) {
        let action = {
            let draft = self
                .draft
                .as_mut()
                .expect("draft is shown only when it exists");
            select_amount_screen::show(ctx, draft)
        };
        match action {
            SelectAction::None => {}
            SelectAction::Back => self.draft = None,
            SelectAction::Proceed(target, speed) => match self.session.select(target, speed) {
                Ok(()) => self.draft = None,
                Err(error) => {
                    if let Some(draft) = self.draft.as_mut() {
                        draft.error = Some(error.to_string());
                    }
                }
            },
        }
    }

    fn show_phase(&mut self, ctx: &egui::Context) {
        let phase = self.session.phase().clone();
        match phase {
            Phase::Standby => {
                if let Some(kind) = standby_screen::show(ctx) {
                    self.draft = Some(AmountDraft::new(kind));
                }
            }
            Phase::WaitingForConnection => {
                let snapshot = self.session.snapshot();
                let actions = connect_screen::show(ctx, &snapshot);
                if actions.back {
                    self.session.request_stop();
                }
                self.apply_simulator(
                    actions.plug,
                    actions.ready,
                    actions.unplug,
                    actions.fault,
                    actions.clear_fault,
                );
            }
            Phase::AwaitingPayment => {
                let selection = self.session.selection();
                let Some(selection) = selection else {
                    self.session.request_stop();
                    return;
                };
                let estimate = self.session.estimate_cost();
                let currency = self.session.config().currency.clone();
                let status = self.session.payment_status();
                let action = payment_screen::show(
                    ctx,
                    PaymentView {
                        selection,
                        estimate,
                        currency: &currency,
                        selected: self.selected_method,
                        status,
                    },
                );
                match action {
                    PaymentAction::None => {}
                    PaymentAction::Back => self.session.request_stop(),
                    PaymentAction::Choose(method) => self.selected_method = Some(method),
                    PaymentAction::Start(method) => {
                        self.selected_method = Some(method);
                        if let Err(error) = self.session.choose_payment(method) {
                            info!(%error, "payment was not accepted");
                        }
                    }
                }
            }
            Phase::Charging | Phase::Stopping { .. } => {
                let stopping = matches!(self.session.phase(), Phase::Stopping { .. });
                let selection = self.session.selection();
                let live = self.session.live_charge();
                let snapshot = self.session.snapshot();
                let (Some(selection), Some(live)) = (selection, live) else {
                    return;
                };
                match charging_screen::show(ctx, selection, &live, &snapshot, stopping) {
                    ChargingAction::None => {}
                    ChargingAction::Stop => self.session.request_stop(),
                    ChargingAction::Unplug => self.session.charger_mut().unplug(),
                    ChargingAction::Fault => self.session.charger_mut().inject_fault(),
                }
            }
            Phase::Complete => {
                let summary = self
                    .session
                    .summary()
                    .expect("complete phase keeps a summary")
                    .clone();
                if complete_screen::show(ctx, &summary) {
                    self.session.acknowledge();
                }
            }
            Phase::Fault {
                message,
                delivered_kwh,
                cost,
            } => {
                let currency = self.session.config().currency.clone();
                match fault_screen::show(ctx, &message, delivered_kwh, cost, &currency) {
                    FaultAction::None => {}
                    FaultAction::Acknowledge => self.session.acknowledge(),
                    FaultAction::ClearFault => self.session.charger_mut().clear_fault(),
                }
            }
        }
    }

    fn apply_simulator(&mut self, plug: bool, ready: bool, unplug: bool, fault: bool, clear: bool) {
        let charger = self.session.charger_mut();
        if clear {
            charger.clear_fault();
        }
        if plug {
            charger.plug_in();
        }
        if ready {
            charger.make_ready();
        }
        if unplug {
            charger.unplug();
        }
        if fault {
            charger.inject_fault();
        }
    }

    fn status_line(&self) -> String {
        let phase = match self.session.phase() {
            Phase::Standby => "Standby",
            Phase::WaitingForConnection => "Waiting for vehicle",
            Phase::AwaitingPayment => "Payment",
            Phase::Charging => "Charging",
            Phase::Stopping { .. } => "Stopping",
            Phase::Complete => "Complete",
            Phase::Fault { .. } => "Fault",
        };
        let contactor = match self.session.snapshot().contactor {
            ContactorCommand::Open => "contactor open",
            ContactorCommand::Closed => "contactor closed",
        };
        format!(
            "SIMULATOR {:.0}x · {phase} · {contactor} · {:.0} {}/kWh",
            self.session.charger().time_scale(),
            self.session.config().tariff_per_kwh,
            self.session.config().currency
        )
    }

    fn repaint_after(&self) -> Duration {
        match self.session.phase() {
            Phase::Charging | Phase::Stopping { .. } | Phase::AwaitingPayment => {
                Duration::from_millis(100)
            }
            _ => Duration::from_millis(250),
        }
    }
}

fn load_splash() -> SplashScreen {
    let path = std::path::PathBuf::from("assets/images/splash_bg.jpg");
    if path.exists() {
        SplashScreen::new().with_background_image(path)
    } else {
        SplashScreen::new()
    }
}
