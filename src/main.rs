use eframe::egui;
// use std::time::Duration;

mod screen;
mod layout;
use screen::{SplashScreen, StandbyScreen};

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    Splash,
    Standby,
}

struct EvChargerApp {
    state: AppState,
    splash_screen: SplashScreen,
    standby_screen: StandbyScreen,
}

impl EvChargerApp {
    fn new() -> Self {
        // 배경 이미지 경로 설정 (선택사항)
        let splash_bg_path = std::path::PathBuf::from("assets/images/splash_bg.jpg");
        let standby_bg_path = std::path::PathBuf::from("assets/images/standby_bg.jpg");
        
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
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(true)
            .with_decorations(true)
            .with_always_on_top()
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "EV Charger",
        options,
        Box::new(|_cc| Ok(Box::new(EvChargerApp::new()))),
    )
}

impl eframe::App for EvChargerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            }
        }
    }
}
