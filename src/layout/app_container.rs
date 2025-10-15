use eframe::egui;

/// 화면 스케일링을 위한 유틸리티 함수
pub fn calculate_scale(ctx: &egui::Context) -> f32 {
    let viewport_rect = ctx.screen_rect();
    let vw = viewport_rect.width();
    let vh = viewport_rect.height();
    let base_w = 800.0;
    let base_h = 600.0;
    (vw / base_w).min(vh / base_h).clamp(0.6, 2.0)
}

