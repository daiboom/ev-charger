use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;
use crate::layout::app_bar::AppBar;
// use crate::layout::stepper::stepper;
use crate::screen::select_amount_screen::ChargeType;
use crate::screen::payment_screen::PaymentMethod;

pub struct CompleteScreen {
    start_time: Instant,
    charge_type: ChargeType,
    charge_amount: f32,
    payment_method: PaymentMethod,
    total_cost: f32,
    charging_duration: Duration,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    return_home_clicked: bool,
    app_bar: AppBar,
}

impl CompleteScreen {
    pub fn new(
        charge_type: ChargeType,
        charge_amount: f32,
        payment_method: PaymentMethod,
        total_cost: f32,
        charging_duration: Duration,
    ) -> Self {
        Self {
            start_time: Instant::now(),
            charge_type,
            charge_amount,
            payment_method,
            total_cost,
            charging_duration,
            background_image_path: None,
            background_image: None,
            return_home_clicked: false,
            app_bar: AppBar::new("Charging Complete"),
        }
    }

    pub fn with_background_image(mut self, image_path: PathBuf) -> Self {
        self.background_image_path = Some(image_path);
        self
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
                        self.background_image = Some(ctx.load_texture("complete_background", color_image, Default::default()));
                    }
                }
            }
        }
    }

    pub fn is_return_home_clicked(&self) -> bool {
        self.return_home_clicked
    }

    pub fn reset_return_home_clicked(&mut self) {
        self.return_home_clicked = false;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.load_background_image(ctx);

        let viewport_rect = ctx.screen_rect();
        let vw = viewport_rect.width();
        let vh = viewport_rect.height();
        let base_w = 800.0;
        let base_h = 600.0;
        let scale = (vw / base_w).min(vh / base_h).clamp(0.6, 2.0);

        show_top_bar(ctx, scale);

        // AppBar 표시
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                self.app_bar.show(ui, scale);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let screen_rect = ui.max_rect();

                // 배경 이미지 또는 색상
                if let Some(ref texture) = self.background_image {
                    ui.painter().image(
                        texture.id(),
                        screen_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                } else {
                    let bg_color = egui::Color32::from_rgba_premultiplied(25, 35, 55, 255);
                    ui.painter().rect_filled(screen_rect, 0.0, bg_color);
                }

                ui.add_space(20.0 * scale);

                // 제목
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0 * scale);

                    // 완료 아이콘
                    ui.add(egui::Label::new(
                        egui::RichText::new("✅")
                            .font(egui::FontId::proportional(48.0 * scale))
                            .color(egui::Color32::from_rgb(100, 255, 100)),
                    ));

                    ui.add_space(15.0 * scale);

                    ui.add(egui::Label::new(
                        egui::RichText::new("Charging Complete!")
                            .font(egui::FontId::proportional(28.0 * scale))
                            .color(egui::Color32::WHITE),
                    ));

                    ui.add_space(30.0 * scale);

                    // 충전 요약 정보
                    let summary_rect = egui::Rect::from_center_size(
                        ui.available_rect_before_wrap().center(),
                        egui::vec2(500.0 * scale, 300.0 * scale),
                    );

                    // 요약 박스 배경
                    ui.painter().rect_filled(
                        summary_rect,
                        egui::CornerRadius::same(15),
                        egui::Color32::from_rgba_premultiplied(40, 50, 70, 200),
                    );

                    ui.painter().rect_stroke(
                        summary_rect,
                        egui::CornerRadius::same(15),
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 255, 100)),
                        egui::StrokeKind::Outside,
                    );

                    // 요약 내용을 직접 그리기
                    let center_x = summary_rect.center().x;
                    let mut y_pos = summary_rect.top() + 30.0 * scale;

                    // 충전 정보
                    let charge_info = match self.charge_type {
                        ChargeType::SpecificWatts(watts) => format!("Charged: {:.0} kW", watts),
                        ChargeType::Percent(percent) => format!("Target: {:.0}%", percent),
                    };

                    ui.painter().text(
                        egui::pos2(center_x, y_pos),
                        egui::Align2::CENTER_CENTER,
                        &charge_info,
                        egui::FontId::proportional(20.0 * scale),
                        egui::Color32::WHITE,
                    );

                    y_pos += 35.0 * scale;

                    // 충전 시간
                    let duration_minutes = self.charging_duration.as_secs() / 60;
                    let duration_seconds = self.charging_duration.as_secs() % 60;
                    let duration_text = format!("Duration: {}:{}", duration_minutes, format!("{:02}", duration_seconds));

                    ui.painter().text(
                        egui::pos2(center_x, y_pos),
                        egui::Align2::CENTER_CENTER,
                        &duration_text,
                        egui::FontId::proportional(18.0 * scale),
                        egui::Color32::from_gray(200),
                    );

                    y_pos += 30.0 * scale;

                    // 결제 방법
                    let payment_text = match self.payment_method {
                        PaymentMethod::CreditCard => "💳 Credit Card",
                        PaymentMethod::MobileApp => "📱 Mobile App",
                        PaymentMethod::RFID => "🔑 RFID Card",
                        PaymentMethod::Membership => "🎫 Membership",
                    };

                    ui.painter().text(
                        egui::pos2(center_x, y_pos),
                        egui::Align2::CENTER_CENTER,
                        &format!("Payment: {}", payment_text),
                        egui::FontId::proportional(18.0 * scale),
                        egui::Color32::from_gray(200),
                    );

                    y_pos += 40.0 * scale;

                    // 총 비용
                    ui.painter().text(
                        egui::pos2(center_x, y_pos),
                        egui::Align2::CENTER_CENTER,
                        "Total Cost",
                        egui::FontId::proportional(16.0 * scale),
                        egui::Color32::from_gray(200),
                    );

                    y_pos += 25.0 * scale;

                    ui.painter().text(
                        egui::pos2(center_x, y_pos),
                        egui::Align2::CENTER_CENTER,
                        &format!("{:.0} KRW", self.total_cost),
                        egui::FontId::proportional(24.0 * scale),
                        egui::Color32::from_rgb(255, 200, 100),
                    );

                    y_pos += 40.0 * scale;

                    // 감사 메시지
                    ui.painter().text(
                        egui::pos2(center_x, y_pos),
                        egui::Align2::CENTER_CENTER,
                        "Thank you for using our charging service!",
                        egui::FontId::proportional(16.0 * scale),
                        egui::Color32::from_gray(180),
                    );

                    ui.add_space(40.0 * scale);

                    // 홈으로 돌아가기 버튼
                    let home_btn = egui::Button::new(
                        egui::RichText::new("Return to Home")
                            .font(egui::FontId::proportional(18.0 * scale))
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(200.0 * scale, 50.0 * scale))
                    .fill(egui::Color32::from_rgb(20, 180, 120))
                    .corner_radius(egui::CornerRadius::same(10));

                    let resp_home = ui.add(home_btn);
                    if resp_home.hovered() || resp_home.is_pointer_button_down_on() {
                        let glow_rect = resp_home.rect.expand(8.0 * scale);
                        let glow_shape = egui::epaint::RectShape::filled(
                            glow_rect,
                            egui::CornerRadius::same((12.0 * scale) as u8),
                            egui::Color32::from_rgba_premultiplied(20, 180, 120, 80),
                        ).with_blur_width(12.0 * scale);
                        ui.painter().add(glow_shape);
                    }
                    if resp_home.clicked() {
                        self.return_home_clicked = true;
                    }

                    ui.add_space(20.0 * scale);

                    // 추가 옵션들
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(20.0 * scale, 0.0);

                        // 영수증 인쇄 버튼
                        let receipt_btn = egui::Button::new(
                            egui::RichText::new("🖨️ Print Receipt")
                                .font(egui::FontId::proportional(14.0 * scale))
                                .color(egui::Color32::WHITE),
                        )
                        .min_size(egui::vec2(120.0 * scale, 35.0 * scale))
                        .fill(egui::Color32::from_rgba_premultiplied(60, 60, 80, 255))
                        .corner_radius(egui::CornerRadius::same(8));

                        if ui.add(receipt_btn).clicked() {
                            println!("Print receipt requested");
                        }

                        // 다시 충전 버튼
                        let recharge_btn = egui::Button::new(
                            egui::RichText::new("🔄 Charge Again")
                                .font(egui::FontId::proportional(14.0 * scale))
                                .color(egui::Color32::WHITE),
                        )
                        .min_size(egui::vec2(120.0 * scale, 35.0 * scale))
                        .fill(egui::Color32::from_rgba_premultiplied(60, 60, 80, 255))
                        .corner_radius(egui::CornerRadius::same(8));

                        if ui.add(recharge_btn).clicked() {
                            println!("Start new charging session");
                        }
                    });
                });
            });

        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
