use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;
// use crate::layout::app_bar::AppBar;
use crate::layout::app_container::calculate_scale;

pub struct ConnectScreen {
    start_time: Instant,
    connection_status: ConnectionStatus,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    proceed_clicked: bool,
    // app_bar: AppBar,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Waiting,      // 전기차 연결 대기
    Connecting,   // 전기차 연결 중
    Verifying,    // 통신 프로토콜 확인 중
    Finalizing,   // 최종 연결 완료 중
    Connected,    // 전기차 연결됨
    Error,        // 연결 오류
}

impl ConnectScreen {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            connection_status: ConnectionStatus::Waiting,
            background_image_path: None,
            background_image: None,
            proceed_clicked: false,
            // app_bar: AppBar::new("Connect Your EV"),
        }
    }

    pub fn with_background_image(mut self, image_path: PathBuf) -> Self {
        self.background_image_path = Some(image_path);
        self
    }

    pub fn is_proceed_clicked(&self) -> bool {
        self.proceed_clicked
    }

    pub fn reset_proceed_clicked(&mut self) {
        self.proceed_clicked = false;
    }

    // pub fn is_back_clicked(&self) -> bool {
        // self.app_bar.is_back_clicked()
    // }

    // pub fn reset_back_clicked(&mut self) {
    //     self.app_bar.reset_back_clicked();
    // }

    pub fn get_connection_status(&self) -> &ConnectionStatus {
        &self.connection_status
    }

    fn load_background_image(&mut self, ctx: &egui::Context) {
        if let Some(ref path) = self.background_image_path {
            if self.background_image.is_none() {
                if let Ok(image_data) = std::fs::read(path) {
                    if let Ok(image) = image::load_from_memory(&image_data) {
                        let rgba_image = image.to_rgba8();
                        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                        let pixels = rgba_image.into_raw();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                        self.background_image = Some(ctx.load_texture("connect_bg", color_image, Default::default()));
                    }
                }
            }
        }
    }

    fn update_connection_status(&mut self) {
        let elapsed = self.start_time.elapsed();
        
        // 실제 EV 충전기 연결 감지 시뮬레이션
        match self.connection_status {
            ConnectionStatus::Waiting => {
                // 1단계: 커넥터 감지 (1초)
                if elapsed > Duration::from_secs(1) {
                    self.connection_status = ConnectionStatus::Connecting;
                }
            }
            ConnectionStatus::Connecting => {
                // 2단계: 전기적 연결 확인 (2초)
                if elapsed > Duration::from_secs(2) {
                    self.connection_status = ConnectionStatus::Verifying;
                }
            }
            ConnectionStatus::Verifying => {
                // 3단계: 통신 프로토콜 확인 (3초)
                if elapsed > Duration::from_secs(3) {
                    self.connection_status = ConnectionStatus::Finalizing;
                }
            }
            ConnectionStatus::Finalizing => {
                // 4단계: 최종 연결 완료 (5초)
                if elapsed > Duration::from_secs(5) {
                    self.connection_status = ConnectionStatus::Connected;
                }
            }
            ConnectionStatus::Connected => {
                // 연결 유지 상태 - 실제로는 지속적인 모니터링
            }
            ConnectionStatus::Error => {
                // 오류 상태 - 재시도 로직 필요
            }
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let scale = calculate_scale(ctx);

        show_top_bar(ctx, scale);

        // AppBar 표시
        // egui::CentralPanel::default()
        //     .frame(egui::Frame::NONE)
        //     .show(ctx, |ui| {
        //         self.app_bar.show(ui, scale);
        //     });

        // 메인 콘텐츠 - "Connect Your Car" 텍스트를 가로 세로 정중앙에 배치
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // 화면 중앙에 정확히 배치하기 위한 레이아웃
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // 세로 중앙 정렬을 위한 공간
                    ui.add_space(ui.available_height() / 2.0 - 20.0 * scale);
                    
                    // 텍스트 추가
                    ui.add(egui::Label::new(
                        egui::RichText::new("Connect Your Car")
                            .font(egui::FontId::proportional(32.0 * scale))
                            .color(egui::Color32::WHITE),
                    ));
                });
            });
    }
}
