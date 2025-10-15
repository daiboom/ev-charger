use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;
use crate::layout::app_bar::AppBar;
use crate::layout::app_container::calculate_scale;
// use crate::layout::stepper::stepper;
use crate::screen::select_amount_screen::ChargeType;

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    MobileApp,
    RFID,
    Membership,
}

pub struct PaymentScreen {
    start_time: Instant,
    charge_type: ChargeType,
    charge_amount: f32,
    selected_payment: Option<PaymentMethod>,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    proceed_clicked: bool,
    app_bar: AppBar,
}

impl PaymentScreen {
    pub fn new(charge_type: ChargeType, charge_amount: f32) -> Self {
        Self {
            start_time: Instant::now(),
            charge_type,
            charge_amount,
            selected_payment: None,
            background_image_path: None,
            background_image: None,
            proceed_clicked: false,
            app_bar: AppBar::new("Select Payment Method").with_back_button(),
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
                        self.background_image = Some(ctx.load_texture("payment_background", color_image, Default::default()));
                    }
                }
            }
        }
    }

    pub fn get_selected_payment(&self) -> Option<PaymentMethod> {
        self.selected_payment.clone()
    }

    pub fn is_proceed_clicked(&self) -> bool {
        self.proceed_clicked
    }

    pub fn reset_proceed_clicked(&mut self) {
        self.proceed_clicked = false;
    }

    pub fn is_back_clicked(&self) -> bool {
        self.app_bar.is_back_clicked()
    }

    pub fn reset_back_clicked(&mut self) {
        self.app_bar.reset_back_clicked();
    }

    fn calculate_cost(&self) -> f32 {
        // Simple cost calculation (assuming 200 won per kWh)
        match self.charge_type {
            ChargeType::SpecificWatts(watts) => watts * 0.2, // 1kW = 200 won
            ChargeType::Percent(percent) => percent * 0.1,   // 1% = 100 won
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.load_background_image(ctx);

        let scale = calculate_scale(ctx);

        show_top_bar(ctx, scale, None);

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

                    ui.add(egui::Label::new(
                        egui::RichText::new("Select Payment Method")
                            .font(egui::FontId::proportional(28.0 * scale))
                            .color(egui::Color32::WHITE),
                    ));

                    ui.add_space(20.0 * scale);

                    // 충전 정보 요약
                    let charge_info = match self.charge_type {
                        ChargeType::SpecificWatts(watts) => format!("Charging: {:.0} kW", watts),
                        ChargeType::Percent(percent) => format!("Target: {:.0}%", percent),
                    };

                    ui.add(egui::Label::new(
                        egui::RichText::new(charge_info)
                            .font(egui::FontId::proportional(18.0 * scale))
                            .color(egui::Color32::from_gray(200)),
                    ));

                    let cost = self.calculate_cost();
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("Estimated Cost: {:.0} KRW", cost))
                            .font(egui::FontId::proportional(18.0 * scale))
                            .color(egui::Color32::from_rgb(255, 200, 100)),
                    ));

                    ui.add_space(30.0 * scale);

                    // 결제 방법 선택
                    let payment_methods = vec![
                        (PaymentMethod::CreditCard, "💳 Credit Card", "Insert or tap your card"),
                        (PaymentMethod::MobileApp, "📱 Mobile App", "Scan QR code with your app"),
                        (PaymentMethod::RFID, "🔑 RFID Card", "Tap your RFID card"),
                        (PaymentMethod::Membership, "🎫 Membership", "Use membership benefits"),
                    ];

                    for (method, title, description) in payment_methods {
                        let is_selected = self.selected_payment == Some(method.clone());
                        let btn_color = if is_selected {
                            egui::Color32::from_rgb(20, 180, 120)
                        } else {
                            egui::Color32::from_rgba_premultiplied(60, 60, 80, 255)
                        };

                        let btn = egui::Button::new("")
                            .min_size(egui::vec2(400.0 * scale, 60.0 * scale))
                            .fill(btn_color)
                            .corner_radius(egui::CornerRadius::same(10));

                        let resp = ui.add(btn);
                        if resp.hovered() || resp.is_pointer_button_down_on() {
                            let glow_rect = resp.rect.expand(6.0 * scale);
                            let glow_shape = egui::epaint::RectShape::filled(
                                glow_rect,
                                egui::CornerRadius::same((12.0 * scale) as u8),
                                egui::Color32::from_rgba_premultiplied(100, 100, 120, 80),
                            ).with_blur_width(10.0 * scale);
                            ui.painter().add(glow_shape);
                        }
                        if resp.clicked() {
                            self.selected_payment = Some(method);
                        }

                        // 버튼 내용을 직접 그리기
                        let button_center = resp.rect.center();
                        let text_pos = egui::pos2(resp.rect.left() + 20.0 * scale, button_center.y - 8.0 * scale);
                        let desc_pos = egui::pos2(resp.rect.left() + 20.0 * scale, button_center.y + 8.0 * scale);
                        
                        ui.painter().text(
                            text_pos,
                            egui::Align2::LEFT_CENTER,
                            title,
                            egui::FontId::proportional(18.0 * scale),
                            egui::Color32::WHITE,
                        );
                        
                        ui.painter().text(
                            desc_pos,
                            egui::Align2::LEFT_CENTER,
                            description,
                            egui::FontId::proportional(14.0 * scale),
                            egui::Color32::from_gray(200),
                        );
                        
                        if is_selected {
                            let check_pos = egui::pos2(resp.rect.right() - 20.0 * scale, button_center.y);
                            ui.painter().text(
                                check_pos,
                                egui::Align2::CENTER_CENTER,
                                "OK",
                                egui::FontId::proportional(24.0 * scale),
                                egui::Color32::WHITE,
                            );
                        }

                        ui.add_space(10.0 * scale);
                    }

                    ui.add_space(30.0 * scale);

                    // 진행 버튼 (결제 방법이 선택되었을 때만 활성화)
                    let can_proceed = self.selected_payment.is_some();
                    let proceed_color = if can_proceed {
                        egui::Color32::from_rgb(20, 180, 120)
                    } else {
                        egui::Color32::from_gray(100)
                    };

                    let proceed_btn = egui::Button::new(
                        egui::RichText::new("Start Charging")
                            .font(egui::FontId::proportional(18.0 * scale))
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(250.0 * scale, 50.0 * scale))
                    .fill(proceed_color)
                    .corner_radius(egui::CornerRadius::same(10));

                    let resp_proceed = ui.add(proceed_btn);
                    if can_proceed && (resp_proceed.hovered() || resp_proceed.is_pointer_button_down_on()) {
                        let glow_rect = resp_proceed.rect.expand(8.0 * scale);
                        let glow_shape = egui::epaint::RectShape::filled(
                            glow_rect,
                            egui::CornerRadius::same((12.0 * scale) as u8),
                            egui::Color32::from_rgba_premultiplied(20, 180, 120, 80),
                        ).with_blur_width(12.0 * scale);
                        ui.painter().add(glow_shape);
                    }
                    if resp_proceed.clicked() && can_proceed {
                        self.proceed_clicked = true;
                    }

                });
            });

        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
