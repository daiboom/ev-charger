use eframe::egui;
use std::path::PathBuf;
use crate::screen::{SplashScreen, StandbyScreen, FullChargeScreen};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Splash,
    Standby,
    FullCharge,
}

pub struct Router {
    pub state: AppState,
    pub splash_screen: SplashScreen,
    pub standby_screen: StandbyScreen,
    pub full_charge_screen: FullChargeScreen,
}

impl Router {
    pub fn new() -> Self {
        // 배경 이미지 경로 설정 (선택사항)
        let splash_bg_path = std::path::PathBuf::from("assets/images/splash_bg.jpg");
        let standby_bg_path = std::path::PathBuf::from("assets/images/standby_bg.jpg");
        let full_charge_bg_path = std::path::PathBuf::from("assets/images/full_charge_bg.jpg");
        
        Self {
            state: AppState::Splash,
            splash_screen: if splash_bg_path.exists() {
                SplashScreen::new().with_background_image(splash_bg_path)
            } else {
                SplashScreen::new()
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
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::Splash => {
                self.splash_screen.show(ctx);
                
                // 스플래시가 끝나면 스탠바이 화면으로 전환
                if self.splash_screen.is_finished() {
                    self.state = AppState::Standby;
                }
            }
            AppState::Standby => {
                self.standby_screen.show(ctx);
                
                // 풀 충전 버튼 클릭 시 풀 충전 화면으로 전환
                if self.standby_screen.is_full_charge_clicked() {
                    self.standby_screen.reset_full_charge_clicked();
                    self.state = AppState::FullCharge;
                }
            }
            AppState::FullCharge => {
                self.full_charge_screen.show(ctx);
                
                // 충전 완료 시 스탠바이 화면으로 돌아가기
                if self.full_charge_screen.is_charging_complete() && 
                   ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.state = AppState::Standby;
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

    // 현재 상태 확인 메서드들
    pub fn is_splash(&self) -> bool {
        self.state == AppState::Splash
    }

    pub fn is_standby(&self) -> bool {
        self.state == AppState::Standby
    }

    pub fn is_full_charge(&self) -> bool {
        self.state == AppState::FullCharge
    }
}
