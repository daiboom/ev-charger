use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::app_bar::AppBar;
use crate::layout::app_container::calculate_scale;

pub struct SplashScreen {
    start_time: Instant,
    fade_progress: f32,
    logo_scale: f32,
    text_alpha: f32,
    show_text: bool,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    // app_bar: AppBar,
}

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            fade_progress: 0.0,
            logo_scale: 0.0,
            text_alpha: 0.0,
            show_text: false,
            background_image_path: None,
            background_image: None,
            // app_bar: AppBar::new("EV Charger"),
        }
    }

    pub fn with_background_image(mut self, image_path: PathBuf) -> Self {
        self.background_image_path = Some(image_path);
        self
    }

    fn load_background_image(&mut self, ctx: &egui::Context) {
        if let Some(ref path) = self.background_image_path {
            if self.background_image.is_none() {
                // Try to load the image
                if let Ok(image_data) = std::fs::read(path) {
                    if let Ok(image) = image::load_from_memory(&image_data) {
                        let rgba_image = image.to_rgba8();
                        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                        let pixels = rgba_image.into_raw();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                        self.background_image = Some(ctx.load_texture("background", color_image, Default::default()));
                    }
                }
            }
        }
    }

    pub fn update_animation(&mut self) {
        let elapsed = self.start_time.elapsed();
        
        // 로고 스케일 애니메이션 (0.5초)
        if elapsed < Duration::from_millis(500) {
            self.logo_scale = (elapsed.as_millis() as f32 / 500.0).min(1.0);
        } else {
            self.logo_scale = 1.0;
        }
        
        // 텍스트 페이드인 (1초 후 시작, 0.8초 동안)
        if elapsed > Duration::from_millis(1000) {
            self.show_text = true;
            let text_start = 1000.0;
            let text_duration = 800.0;
            let text_progress = ((elapsed.as_millis() as f32 - text_start) / text_duration).min(1.0);
            self.text_alpha = text_progress;
        }
        
        // 전체 페이드아웃 (3초 후 시작, 1초 동안)
        if elapsed > Duration::from_millis(3000) {
            let fade_start = 3000.0;
            let fade_duration = 1000.0;
            let fade_progress = ((elapsed.as_millis() as f32 - fade_start) / fade_duration).min(1.0);
            self.fade_progress = fade_progress;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() > Duration::from_millis(4000)
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.update_animation();
        self.load_background_image(ctx);

        let scale = calculate_scale(ctx);

        // 전체 화면을 투명하게 설정
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let screen_rect = ui.max_rect();
                
                // 배경 이미지 또는 색상
                if let Some(ref texture) = self.background_image {
                    // 배경 이미지 그리기
                    ui.painter().image(
                        texture.id(),
                        screen_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                } else {
                    // 배경 그라데이션 (fallback)
                    let bg_color = egui::Color32::from_rgba_premultiplied(20, 30, 50, 255);
                    ui.painter().rect_filled(screen_rect, 0.0, bg_color);
                }
                
                // 페이드아웃 효과
                if self.fade_progress > 0.0 {
                    let fade_alpha = (255.0 * (1.0 - self.fade_progress)) as u8;
                    let fade_color = egui::Color32::from_rgba_premultiplied(0, 0, 0, fade_alpha);
                    ui.painter().rect_filled(screen_rect, 0.0, fade_color);
                }
                
                // (Removed charger-themed logo and branding)
                
                // 로딩 바
                if self.show_text {
                    let loading_alpha = (255.0 * self.text_alpha) as u8;
                    let loading_color = egui::Color32::from_rgba_premultiplied(0, 120, 215, loading_alpha);
                    
                    let loading_rect = egui::Rect::from_center_size(
                        egui::pos2(screen_rect.center().x, screen_rect.center().y + 150.0),
                        egui::vec2(200.0, 4.0),
                    );
                    
                    // 로딩 바 배경
                    let bg_color = egui::Color32::from_rgba_premultiplied(100, 100, 100, loading_alpha);
                    ui.painter().rect_filled(loading_rect, 2.0, bg_color);
                    
                    // 로딩 바 진행률
                    let progress = (self.start_time.elapsed().as_millis() as f32 / 4000.0).min(1.0);
                    let progress_rect = egui::Rect::from_min_size(
                        loading_rect.min,
                        egui::vec2(loading_rect.width() * progress, loading_rect.height()),
                    );
                    ui.painter().rect_filled(progress_rect, 2.0, loading_color);
                }
                
                // 버전 정보
                if self.show_text {
                    let version_alpha = (255.0 * self.text_alpha * 0.7) as u8;
                    let version_color = egui::Color32::from_rgba_premultiplied(200, 200, 200, version_alpha);
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(30.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new("Version 1.0.0")
                                .font(egui::FontId::proportional(12.0))
                                .color(version_color),
                        ));
                    });
                }
            });

        // 60fps로 업데이트
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
