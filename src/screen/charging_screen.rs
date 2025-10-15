use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;
use crate::layout::app_bar::AppBar;
use crate::layout::app_container::calculate_scale;
// use crate::layout::stepper::stepper;
use crate::screen::select_amount_screen::ChargeType;
use crate::screen::payment_screen::PaymentMethod;

pub struct ChargingScreen {
    start_time: Instant,
    charge_type: ChargeType,
    charge_amount: f32,
    payment_method: PaymentMethod,
    current_battery_level: f32,
    charging_power: f32,
    estimated_time: Duration,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    is_charging_complete: bool,
    app_bar: AppBar,
}

impl ChargingScreen {
    pub fn new(charge_type: ChargeType, charge_amount: f32, payment_method: PaymentMethod) -> Self {
        Self {
            start_time: Instant::now(),
            charge_type,
            charge_amount,
            payment_method,
            current_battery_level: 0.0,
            charging_power: 0.0,
            estimated_time: Duration::from_secs(0),
            background_image_path: None,
            background_image: None,
            is_charging_complete: false,
            app_bar: AppBar::new("Charging in Progress").with_back_button(),
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
                        self.background_image = Some(ctx.load_texture("charging_background", color_image, Default::default()));
                    }
                }
            }
        }
    }

    pub fn update_charging(&mut self) {
        let elapsed = self.start_time.elapsed();
        
        match self.charge_type {
            ChargeType::SpecificWatts(target_watts) => {
                // 특정 와트 충전 시뮬레이션
                let total_charge_time = Duration::from_secs_f32(target_watts * 6.0); // 1kW당 6초
                self.current_battery_level = (elapsed.as_secs_f32() / total_charge_time.as_secs_f32()).min(1.0);
                self.charging_power = target_watts;
                
                if self.current_battery_level < 1.0 {
                    let remaining_ratio = 1.0 - self.current_battery_level;
                    self.estimated_time = Duration::from_secs_f32(remaining_ratio * total_charge_time.as_secs_f32());
                } else {
                    self.estimated_time = Duration::from_secs(0);
                    self.is_charging_complete = true;
                }
            }
            ChargeType::Percent(target_percent) => {
                // 퍼센트 충전 시뮬레이션
                let total_charge_time = Duration::from_secs_f32(target_percent * 3.0); // 1%당 3초
                self.current_battery_level = (elapsed.as_secs_f32() / total_charge_time.as_secs_f32()).min(1.0);
                self.charging_power = 50.0; // 고정 전력
                
                if self.current_battery_level < 1.0 {
                    let remaining_ratio = 1.0 - self.current_battery_level;
                    self.estimated_time = Duration::from_secs_f32(remaining_ratio * total_charge_time.as_secs_f32());
                } else {
                    self.estimated_time = Duration::from_secs(0);
                    self.is_charging_complete = true;
                }
            }
        }
    }

    pub fn is_charging_complete(&self) -> bool {
        self.is_charging_complete
    }

    pub fn is_back_clicked(&self) -> bool {
        self.app_bar.is_back_clicked()
    }

    pub fn reset_back_clicked(&mut self) {
        self.app_bar.reset_back_clicked();
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.update_charging();
        self.load_background_image(ctx);

        let scale = calculate_scale(ctx);

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

                    let title = if self.is_charging_complete {
                        "Charging Complete!"
                    } else {
                        "Charging in Progress"
                    };

                    ui.add(egui::Label::new(
                        egui::RichText::new(title)
                            .font(egui::FontId::proportional(28.0 * scale))
                            .color(egui::Color32::WHITE),
                    ));

                    ui.add_space(20.0 * scale);

                    // 배터리 프로그레스 바
                    let progress_rect = egui::Rect::from_center_size(
                        ui.available_rect_before_wrap().center(),
                        egui::vec2(400.0 * scale, 25.0 * scale),
                    );

                    // 배경 바
                    ui.painter().rect_filled(
                        progress_rect,
                        egui::CornerRadius::same(12),
                        egui::Color32::from_rgba_premultiplied(60, 60, 60, 255),
                    );

                    // 프로그레스 바
                    let progress_width = progress_rect.width() * self.current_battery_level;
                    let progress_rect_filled = egui::Rect::from_min_size(
                        progress_rect.min,
                        egui::vec2(progress_width, progress_rect.height()),
                    );

                    let progress_color = if self.current_battery_level < 0.2 {
                        egui::Color32::from_rgb(255, 100, 100) // 빨간색
                    } else if self.current_battery_level < 0.5 {
                        egui::Color32::from_rgb(255, 200, 100) // 주황색
                    } else {
                        egui::Color32::from_rgb(100, 255, 100) // 초록색
                    };

                    ui.painter().rect_filled(
                        progress_rect_filled,
                        egui::CornerRadius::same(12),
                        progress_color,
                    );

                    // 프로그레스 바 테두리
                    ui.painter().rect_stroke(
                        progress_rect,
                        egui::CornerRadius::same(12),
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                        egui::StrokeKind::Outside,
                    );

                    ui.add_space(20.0 * scale);

                    // 배터리 레벨 텍스트
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{:.1}%", self.current_battery_level * 100.0))
                            .font(egui::FontId::proportional(24.0 * scale))
                            .color(egui::Color32::WHITE),
                    ));

                    ui.add_space(30.0 * scale);

                    // 충전 정보
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(40.0 * scale, 0.0);

                        // 충전 전력
                        ui.vertical_centered(|ui| {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Charging Power")
                                    .font(egui::FontId::proportional(16.0 * scale))
                                    .color(egui::Color32::from_gray(200)),
                            ));
                            ui.add_space(5.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new(format!("{:.1} kW", self.charging_power))
                                    .font(egui::FontId::proportional(20.0 * scale))
                                    .color(egui::Color32::WHITE),
                            ));
                        });

                        // 남은 시간
                        ui.vertical_centered(|ui| {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Estimated Time")
                                    .font(egui::FontId::proportional(16.0 * scale))
                                    .color(egui::Color32::from_gray(200)),
                            ));
                            ui.add_space(5.0 * scale);
                            let time_text = if self.estimated_time.as_secs() == 0 {
                                if self.is_charging_complete {
                                    "Complete!".to_string()
                                } else {
                                    "Calculating...".to_string()
                                }
                            } else {
                                let minutes = self.estimated_time.as_secs() / 60;
                                let seconds = self.estimated_time.as_secs() % 60;
                                format!("{}:{}", minutes, format!("{:02}", seconds))
                            };
                            ui.add(egui::Label::new(
                                egui::RichText::new(time_text)
                                    .font(egui::FontId::proportional(20.0 * scale))
                                    .color(egui::Color32::WHITE),
                            ));
                        });
                    });

                    ui.add_space(30.0 * scale);

                    // 결제 방법 표시
                    let payment_text = match self.payment_method {
                        PaymentMethod::CreditCard => "💳 Credit Card",
                        PaymentMethod::MobileApp => "📱 Mobile App",
                        PaymentMethod::RFID => "🔑 RFID Card",
                        PaymentMethod::Membership => "🎫 Membership",
                    };

                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("Payment: {}", payment_text))
                            .font(egui::FontId::proportional(16.0 * scale))
                            .color(egui::Color32::from_gray(200)),
                    ));

                    ui.add_space(40.0 * scale);

                    // 완료 버튼 (충전이 완료되었을 때만 표시)
                    if self.is_charging_complete {
                        let complete_btn = egui::Button::new(
                            egui::RichText::new("View Summary")
                                .font(egui::FontId::proportional(18.0 * scale))
                                .color(egui::Color32::WHITE),
                        )
                        .min_size(egui::vec2(200.0 * scale, 50.0 * scale))
                        .fill(egui::Color32::from_rgb(20, 180, 120))
                        .corner_radius(egui::CornerRadius::same(10));

                        let resp_complete = ui.add(complete_btn);
                        if resp_complete.hovered() || resp_complete.is_pointer_button_down_on() {
                            let glow_rect = resp_complete.rect.expand(8.0 * scale);
                            let glow_shape = egui::epaint::RectShape::filled(
                                glow_rect,
                                egui::CornerRadius::same((12.0 * scale) as u8),
                                egui::Color32::from_rgba_premultiplied(20, 180, 120, 80),
                            ).with_blur_width(12.0 * scale);
                            ui.painter().add(glow_shape);
                        }
                        if resp_complete.clicked() {
                            println!("Proceed to completion summary");
                        }
                    }

                });
            });

        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
