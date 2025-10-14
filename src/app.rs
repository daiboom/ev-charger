//! 메인 애플리케이션 로직

use crate::prelude::*;
use crate::ui::screens::charging::ChargingScreen;
use crate::ui::screens::complete::CompleteScreen;
use crate::ui::screens::error::ErrorScreen;
use crate::ui::screens::home::HomeScreen;
use crate::ui::screens::payment::PaymentScreen;
use crate::ui::theme::Theme;
use tokio::sync::mpsc;

#[derive(Debug, PartialEq, Clone)]
pub enum AppState {
    Initializing,
    Home,
    Charging,
    Payment,
    Complete,
    Error(String),
}

pub struct EvChargerApp {
    communication_events: Option<mpsc::UnboundedReceiver<CommunicationEvent>>,
    state: AppState,
    frame_count: u64,
    theme: Theme,
    home_screen: HomeScreen,
    charging_screen: ChargingScreen,
    payment_screen: PaymentScreen,
    complete_screen: CompleteScreen,
    error_screen: ErrorScreen,
}

#[derive(Debug, Clone)]
pub enum CommunicationEvent {
    PowerMeterReading(f32),
    CanMessage(Vec<u8>),
    OcppMessage(String),
    Shutdown,
}

impl EvChargerApp {
    pub fn new(config: Config, comm_events: mpsc::UnboundedReceiver<CommunicationEvent>) -> Self {
        Self {
            communication_events: Some(comm_events),
            state: AppState::Initializing,
            frame_count: 0,
            theme: Theme::dark(),
            home_screen: HomeScreen::new(),
            charging_screen: ChargingScreen::new(),
            payment_screen: PaymentScreen::new(),
            complete_screen: CompleteScreen::new(),
            error_screen: ErrorScreen::new("Unknown error".to_string()),
        }
    }
}

impl eframe::App for EvChargerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        self.theme.apply_to_context(ctx);

        // 통신 이벤트 처리
        if let Some(ref mut rx) = self.communication_events {
            while let Ok(event) = rx.try_recv() {
                match event {
                    CommunicationEvent::PowerMeterReading(power) => {
                        info!("전력 측정값: {}kW", power);
                    }
                    CommunicationEvent::CanMessage(data) => {
                        debug!("CAN 메시지 수신: {:02X?}", data);
                    }
                    CommunicationEvent::OcppMessage(msg) => {
                        info!("OCPP 메시지: {}", msg);
                    }
                    CommunicationEvent::Shutdown => {
                        info!("종료 이벤트를 수신하여 UI를 닫습니다.");
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            }
        }

        // 상태 기반 UI 렌더링
        

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme.colors.background))
            .show(ctx, |ui| {
                let new_state = match self.state.clone() {
                    AppState::Initializing => {
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(egui::Label::new(
                                    egui::RichText::new("EV Charger Kiosk")
                                        .font(egui::FontId::proportional(self.theme.fonts.heading.size * 1.5))
                                        .color(self.theme.colors.primary),
                                ));
                                ui.add_space(self.theme.sizes.spacing * 4.0);

                                let progress = (self.frame_count as f32 / 120.0).min(1.0);
                                ui.add(egui::ProgressBar::new(progress).show_percentage().desired_width(ui.available_width()));
                                ui.add_space(self.theme.sizes.spacing * 2.0);
                                ui.label(egui::RichText::new("시스템이 초기화 중입니다...").font(self.theme.fonts.body.clone()).color(self.theme.colors.text_primary));
                            });
                        });
                        // Use a simple timer to simulate initialization
                        if self.frame_count > 120 {
                            AppState::Home
                        } else {
                            self.state.clone()
                        }
                    }
                    AppState::Home => {
                        if self.home_screen.show(ui, &self.theme) {
                            AppState::Charging
                        } else {
                            self.state.clone()
                        }
                    }
                    AppState::Charging => {
                        if self.charging_screen.show(ui, &self.theme) {
                            AppState::Payment
                        }
                        else {
                            self.state.clone()
                        }
                    }
                    AppState::Payment => {
                        if self.payment_screen.show(ui, &self.theme) {
                            AppState::Complete
                        }
                        else {
                            self.state.clone()
                        }
                    }
                    AppState::Complete => {
                        if self.complete_screen.show(ui, &self.theme) {
                            AppState::Home
                        }
                        else {
                            self.state.clone()
                        }
                    }
                    AppState::Error(ref msg) => {
                        let mut error_screen = ErrorScreen::new(msg.clone());
                        if error_screen.show(ui, &self.theme) {
                            AppState::Home
                        }
                        else {
                            self.state.clone()
                        }
                    }
                };
                self.state = new_state;
            });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
