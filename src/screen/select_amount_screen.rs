use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;
use crate::layout::app_bar::AppBar;
use crate::layout::app_container::calculate_scale;
// use crate::layout::stepper::stepper;

#[derive(Debug, Clone, PartialEq)]
pub enum ChargeType {
    SpecificWatts(f32),  // 특정 와트로 충전
    Percent(f32),        // 특정 퍼센트까지 충전
}

pub struct SelectAmountScreen {
    start_time: Instant,
    charge_type: ChargeType,
    input_value: String,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    proceed_clicked: bool,
    app_bar: AppBar,
}

impl SelectAmountScreen {
    pub fn new(charge_type: ChargeType) -> Self {
        let initial_value = match charge_type {
            ChargeType::SpecificWatts(_) => "50".to_string(),
            ChargeType::Percent(_) => "80".to_string(),
        };

        let title = match charge_type {
            ChargeType::SpecificWatts(_) => "Select Charging Amount(kW)",
            ChargeType::Percent(_) => "Select Target Battery Level",
        };

        Self {
            start_time: Instant::now(),
            charge_type,
            input_value: initial_value,
            background_image_path: None,
            background_image: None,
            proceed_clicked: false,
            app_bar: AppBar::new(title).with_back_button(),
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
                        self.background_image = Some(ctx.load_texture("select_amount_background", color_image, Default::default()));
                    }
                }
            }
        }
    }

    pub fn get_charge_type(&self) -> ChargeType {
        self.charge_type.clone()
    }

    pub fn get_input_value(&self) -> f32 {
        self.input_value.parse().unwrap_or(0.0)
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

    pub fn show(&mut self, ctx: &egui::Context) {
        self.load_background_image(ctx);

        let scale = calculate_scale(ctx);

        show_top_bar(ctx, scale);

       
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
                    ui.add_space(40.0 * scale);

                    let title = match self.charge_type {
                        ChargeType::SpecificWatts(_) => "Select Charging Amount (kW)",
                        ChargeType::Percent(_) => "Select Target Battery Level (%)",
                    };

                    ui.add(egui::Label::new(
                        egui::RichText::new(title)
                            .font(egui::FontId::proportional(28.0 * scale))
                            .color(egui::Color32::WHITE),
                    ));

                    ui.add_space(30.0 * scale);

                    // 입력 필드
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(20.0 * scale, 0.0);

                        // 숫자 입력 필드
                        let input_width = 200.0 * scale;
                        let input_height = 50.0 * scale;

                        let text_edit = egui::TextEdit::singleline(&mut self.input_value)
                            .font(egui::FontId::proportional(24.0 * scale))
                            .desired_width(input_width)
                            .desired_rows(1);

                        let _response = ui.add_sized([input_width, input_height], text_edit);

                        // 단위 표시
                        let unit = match self.charge_type {
                            ChargeType::SpecificWatts(_) => "kW",
                            ChargeType::Percent(_) => "%",
                        };

                        ui.vertical_centered(|ui| {
                            ui.add_space(15.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new(unit)
                                    .font(egui::FontId::proportional(20.0 * scale))
                                    .color(egui::Color32::WHITE),
                            ));
                        });
                    });

                    ui.add_space(40.0 * scale);

                    // 미리 설정된 옵션들
                    let preset_options = match self.charge_type {
                        ChargeType::SpecificWatts(_) => vec![
                            ("25 kW", "25"),
                            ("50 kW", "50"),
                            ("75 kW", "75"),
                            ("100 kW", "100"),
                        ],
                        ChargeType::Percent(_) => vec![
                            ("50%", "50"),
                            ("80%", "80"),
                            ("90%", "90"),
                            ("100%", "100"),
                        ],
                    };

                    ui.add(egui::Label::new(
                        egui::RichText::new("Quick Select")
                            .font(egui::FontId::proportional(18.0 * scale))
                            .color(egui::Color32::from_gray(200)),
                    ));

                    ui.add_space(15.0 * scale);

                    // 프리셋 버튼들
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(15.0 * scale, 0.0);

                        for (label, value) in preset_options {
                            let btn = egui::Button::new(
                                egui::RichText::new(label)
                                    .font(egui::FontId::proportional(16.0 * scale))
                                    .color(egui::Color32::WHITE),
                            )
                            .min_size(egui::vec2(80.0 * scale, 40.0 * scale))
                            .fill(egui::Color32::from_rgba_premultiplied(60, 60, 80, 255))
                            .corner_radius(egui::CornerRadius::same(8));

                            let resp = ui.add(btn);
                            if resp.hovered() || resp.is_pointer_button_down_on() {
                                let glow_rect = resp.rect.expand(4.0 * scale);
                                let glow_shape = egui::epaint::RectShape::filled(
                                    glow_rect,
                                    egui::CornerRadius::same((8.0 * scale) as u8),
                                    egui::Color32::from_rgba_premultiplied(100, 100, 120, 80),
                                ).with_blur_width(8.0 * scale);
                                ui.painter().add(glow_shape);
                            }
                            if resp.clicked() {
                                self.input_value = value.to_string();
                            }
                        }
                    });

                    ui.add_space(50.0 * scale);

                    // 진행 버튼
                    let proceed_btn = egui::Button::new(
                        egui::RichText::new("Proceed to Payment")
                            .font(egui::FontId::proportional(18.0 * scale))
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(250.0 * scale, 50.0 * scale))
                    .fill(egui::Color32::from_rgb(20, 180, 120))
                    .corner_radius(egui::CornerRadius::same(10));

                    let resp_proceed = ui.add(proceed_btn);
                    if resp_proceed.hovered() || resp_proceed.is_pointer_button_down_on() {
                        let glow_rect = resp_proceed.rect.expand(8.0 * scale);
                        let glow_shape = egui::epaint::RectShape::filled(
                            glow_rect,
                            egui::CornerRadius::same((12.0 * scale) as u8),
                            egui::Color32::from_rgba_premultiplied(20, 180, 120, 80),
                        ).with_blur_width(12.0 * scale);
                        ui.painter().add(glow_shape);
                    }
                    if resp_proceed.clicked() {
                        self.proceed_clicked = true;
                    }

                });
            });

        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
