use eframe::egui;
use std::path::PathBuf;
use std::time::Duration;
use crate::screen::{
    SplashScreen, ConnectScreen, StandbyScreen, FullChargeScreen,
    SelectAmountScreen, PaymentScreen, ChargingScreen, CompleteScreen,
    ChargeType, PaymentMethod
};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Splash,
    Connect(ChargeType),
    Standby,
    FullCharge,
    SelectAmount(ChargeType),
    Payment(ChargeType, f32),
    Charging(ChargeType, f32, PaymentMethod),
    Complete(ChargeType, f32, PaymentMethod, f32, Duration),
}

pub struct Router {
    pub state: AppState,
    pub splash_screen: SplashScreen,
    pub connect_screen: ConnectScreen,
    pub standby_screen: StandbyScreen,
    pub full_charge_screen: FullChargeScreen,
    pub select_amount_screen: Option<SelectAmountScreen>,
    pub payment_screen: Option<PaymentScreen>,
    pub charging_screen: Option<ChargingScreen>,
    pub complete_screen: Option<CompleteScreen>,
}

impl Router {
    pub fn new() -> Self {
        // 배경 이미지 경로 설정 (선택사항)
        let splash_bg_path = std::path::PathBuf::from("assets/images/splash_bg.jpg");
        let connect_bg_path = std::path::PathBuf::from("assets/images/connect_bg.jpg");
        let standby_bg_path = std::path::PathBuf::from("assets/images/standby_bg.jpg");
        let full_charge_bg_path = std::path::PathBuf::from("assets/images/full_charge_bg.jpg");
        
        Self {
            state: AppState::Splash,
            splash_screen: if splash_bg_path.exists() {
                SplashScreen::new().with_background_image(splash_bg_path)
            } else {
                SplashScreen::new()
            },
            connect_screen: if connect_bg_path.exists() {
                ConnectScreen::new().with_background_image(connect_bg_path)
            } else {
                ConnectScreen::new()
            },
            standby_screen: if standby_bg_path.exists() {
                StandbyScreen::new().with_background_image(standby_bg_path)
            } else {
                StandbyScreen::new()
            },
            full_charge_screen: if full_charge_bg_path.exists() {
                FullChargeScreen::new().with_background_image(full_charge_bg_path)
            } else {
                FullChargeScreen::new()
            },
            select_amount_screen: None,
            payment_screen: None,
            charging_screen: None,
            complete_screen: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_state = self.state.clone();
        match current_state {
            AppState::Splash => {
                self.splash_screen.show(ctx);
                
                // 스플래시가 끝나면 연결 화면으로 전환
                if self.splash_screen.is_finished() {
                    self.state = AppState::Connect(ChargeType::SpecificWatts(0.0));
                }
            }
            AppState::Connect(charge_type) => {
                self.connect_screen.show(ctx);
                // 연결 화면에서는 자동으로 다음 단계로 진행하거나 뒤로가기 처리
                // 현재는 단순히 화면만 표시
            }
            AppState::Standby => {
                self.standby_screen.show(ctx);
                
                // 풀 충전 버튼 클릭 시 풀 충전 화면으로 전환
                if self.standby_screen.is_full_charge_clicked() {
                    self.standby_screen.reset_full_charge_clicked();
                    self.state = AppState::FullCharge;
                }
                 
                // 특정 와트 충전 버튼 클릭 시 연결 화면으로 전환
                if self.standby_screen.is_specific_watts_clicked() {
                    self.standby_screen.reset_specific_watts_clicked();
                    self.state = AppState::Connect(ChargeType::SpecificWatts(0.0));
                }
                 
                // 퍼센트 충전 버튼 클릭 시 연결 화면으로 전환
                if self.standby_screen.is_percent_clicked() {
                    self.standby_screen.reset_percent_clicked();
                    self.state = AppState::Connect(ChargeType::Percent(0.0));
                }
            }
            AppState::FullCharge => {
                self.full_charge_screen.show(ctx);
                
                // 뒤로가기 버튼 클릭 시 스탠바이 화면으로 돌아가기
                if self.full_charge_screen.is_back_clicked() {
                    self.full_charge_screen.reset_back_clicked();
                    self.state = AppState::Standby;
                }
                
                // 충전 완료 시 스탠바이 화면으로 돌아가기
                if self.full_charge_screen.is_charging_complete() {
                    self.state = AppState::Standby;
                }
            }
            AppState::SelectAmount(charge_type) => {
                if let Some(ref mut screen) = self.select_amount_screen {
                    screen.show(ctx);
                    
                    if screen.is_proceed_clicked() {
                        screen.reset_proceed_clicked();
                        let amount = screen.get_input_value();
                        self.state = AppState::Payment(charge_type.clone(), amount);
                    }
                    
                    if screen.is_back_clicked() {
                        screen.reset_back_clicked();
                        self.state = AppState::Standby;
                    }
                }
            }
            AppState::Payment(charge_type, amount) => {
                if let Some(ref mut screen) = self.payment_screen {
                    screen.show(ctx);
                    
                    if screen.is_proceed_clicked() {
                        screen.reset_proceed_clicked();
                        if let Some(payment_method) = screen.get_selected_payment() {
                            self.state = AppState::Charging(charge_type.clone(), amount, payment_method);
                        }
                    }
                    
                    if screen.is_back_clicked() {
                        screen.reset_back_clicked();
                        self.state = AppState::SelectAmount(charge_type.clone());
                    }
                }
            }
            AppState::Charging(charge_type, amount, payment_method) => {
                if let Some(ref mut screen) = self.charging_screen {
                    screen.show(ctx);
                    
                    if screen.is_charging_complete() {
                        // 충전 완료 시 완료 화면으로 전환
                        let total_cost = match charge_type {
                            ChargeType::SpecificWatts(watts) => watts * 0.2,
                            ChargeType::Percent(percent) => percent * 0.1,
                        };
                        let charging_duration = Duration::from_secs(300); // 5분 시뮬레이션
                        self.state = AppState::Complete(
                            charge_type.clone(),
                            amount,
                            payment_method.clone(),
                            total_cost,
                            charging_duration,
                        );
                    }
                    
                    if screen.is_back_clicked() {
                        screen.reset_back_clicked();
                        self.state = AppState::Payment(charge_type.clone(), amount);
                    }
                }
            }
            AppState::Complete(_, _, _, _, _) => {
                if let Some(ref mut screen) = self.complete_screen {
                    screen.show(ctx);
                    
                    if screen.is_return_home_clicked() {
                        screen.reset_return_home_clicked();
                        self.state = AppState::Standby;
                    }
                }
            }
        }
    }

    // 화면 전환을 위한 헬퍼 메서드들
    pub fn go_to_standby(&mut self) {
        self.state = AppState::Standby;
    }

    pub fn go_to_full_charge(&mut self) {
        self.state = AppState::FullCharge;
    }

    pub fn go_to_splash(&mut self) {
        self.state = AppState::Splash;
    }

    pub fn go_to_select_amount(&mut self, charge_type: ChargeType) {
        let bg_path = std::path::PathBuf::from("assets/images/select_amount_bg.jpg");
        self.select_amount_screen = Some(
            if bg_path.exists() {
                SelectAmountScreen::new(charge_type.clone()).with_background_image(bg_path)
            } else {
                SelectAmountScreen::new(charge_type.clone())
            }
        );
        self.state = AppState::SelectAmount(charge_type);
    }

    pub fn go_to_payment(&mut self, charge_type: ChargeType, amount: f32) {
        let bg_path = std::path::PathBuf::from("assets/images/payment_bg.jpg");
        self.payment_screen = Some(
            if bg_path.exists() {
                PaymentScreen::new(charge_type.clone(), amount).with_background_image(bg_path)
            } else {
                PaymentScreen::new(charge_type.clone(), amount)
            }
        );
        self.state = AppState::Payment(charge_type, amount);
    }

    pub fn go_to_charging(&mut self, charge_type: ChargeType, amount: f32, payment_method: PaymentMethod) {
        let bg_path = std::path::PathBuf::from("assets/images/charging_bg.jpg");
        self.charging_screen = Some(
            if bg_path.exists() {
                ChargingScreen::new(charge_type.clone(), amount, payment_method.clone()).with_background_image(bg_path)
            } else {
                ChargingScreen::new(charge_type.clone(), amount, payment_method.clone())
            }
        );
        self.state = AppState::Charging(charge_type, amount, payment_method);
    }

    pub fn go_to_complete(&mut self, charge_type: ChargeType, amount: f32, payment_method: PaymentMethod, total_cost: f32, duration: Duration) {
        let bg_path = std::path::PathBuf::from("assets/images/complete_bg.jpg");
        self.complete_screen = Some(
            if bg_path.exists() {
                CompleteScreen::new(charge_type.clone(), amount, payment_method.clone(), total_cost, duration).with_background_image(bg_path)
            } else {
                CompleteScreen::new(charge_type.clone(), amount, payment_method.clone(), total_cost, duration)
            }
        );
        self.state = AppState::Complete(charge_type, amount, payment_method, total_cost, duration);
    }

    // 현재 상태 확인 메서드들
    pub fn is_splash(&self) -> bool {
        matches!(self.state, AppState::Splash)
    }

    pub fn is_standby(&self) -> bool {
        matches!(self.state, AppState::Standby)
    }

    pub fn is_full_charge(&self) -> bool {
        matches!(self.state, AppState::FullCharge)
    }
}
