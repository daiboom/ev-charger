use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;

pub struct StandbyScreen {
    start_time: Instant,
    animation_offset: f32,
    pulse_scale: f32,
    show_instructions: bool,
    instruction_alpha: f32,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    full_charge_clicked: bool,
    specific_watts_clicked: bool,
    percent_clicked: bool,
}

impl StandbyScreen {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            animation_offset: 0.0,
            pulse_scale: 1.0,
            show_instructions: true, // Always show instructions/buttons
            instruction_alpha: 1.0,  // Always fully opaque
            background_image_path: None,
            background_image: None,
            full_charge_clicked: false,
            specific_watts_clicked: false,
            percent_clicked: false,
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
                        self.background_image = Some(ctx.load_texture("standby_background", color_image, Default::default()));
                    }
                }
            }
        }
    }

    pub fn update_animation(&mut self) {
        let elapsed = self.start_time.elapsed();
        
        // 애니메이션 오프셋 업데이트
        self.animation_offset = elapsed.as_secs_f32() * 2.0;
        
        // 펄스 애니메이션 (1.5초 주기)
        let pulse_progress = (elapsed.as_secs_f32() / 1.5) % 1.0;
        self.pulse_scale = 1.0 + 0.1 * (pulse_progress * 2.0 * std::f32::consts::PI).sin().abs();
        
        // 항상 버튼 표시 (대기 없이)
        self.show_instructions = true;
        self.instruction_alpha = 1.0;
    }

    pub fn is_full_charge_clicked(&self) -> bool {
        self.full_charge_clicked
    }

    pub fn reset_full_charge_clicked(&mut self) {
        self.full_charge_clicked = false;
    }

    pub fn is_specific_watts_clicked(&self) -> bool {
        self.specific_watts_clicked
    }

    pub fn reset_specific_watts_clicked(&mut self) {
        self.specific_watts_clicked = false;
    }

    pub fn is_percent_clicked(&self) -> bool {
        self.percent_clicked
    }

    pub fn reset_percent_clicked(&mut self) {
        self.percent_clicked = false;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.update_animation();
        self.load_background_image(ctx);

        // Determine responsive scale based on viewport size
        let viewport_rect = ctx.screen_rect();
        let vw = viewport_rect.width();
        let vh = viewport_rect.height();
        let base_w = 800.0;
        let base_h = 600.0;
        let scale = (vw / base_w).min(vh / base_h).clamp(0.6, 2.0);

        // Top bar (split)
        show_top_bar(ctx, scale);

        // Bottom bar (split)
        // show_bottom_bar(ctx, scale);

        // 중앙 콘텐츠
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                
                // Standby screen content
                // Stepper: Select -> Payment -> Charging -> Complete
                ui.add_space(8.0 * scale);
                // stepper(
                //     ui,
                //     &["Select amount", "Payment method", "Charging", "Complete"],
                //     0,
                //     scale,
                // );
                
                // spacing under top area
                ui.add_space(20.0);
                
                // 안내 메시지
                if self.show_instructions {
                    let instruction_alpha = (255.0 * self.instruction_alpha) as u8;
                    let _instruction_color = egui::Color32::from_rgba_premultiplied(255, 255, 255, instruction_alpha);
                    
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0 * scale);

                        let btn_size = egui::vec2(240.0 * scale, 56.0 * scale);
                        
                        ui.vertical_centered(|ui| {
                            let specific_btn = egui::Button::new(
                                egui::RichText::new("Charge by specific watts")
                                    .font(egui::FontId::proportional(16.0 * scale))
                                    .color(egui::Color32::WHITE),
                            )
                            .min_size(btn_size)
                            .fill(egui::Color32::from_rgba_premultiplied(35, 140, 240, instruction_alpha))
                            .corner_radius(egui::CornerRadius::same(10));

                            let resp_spec = ui.add(specific_btn);
                            if resp_spec.hovered() || resp_spec.is_pointer_button_down_on() {
                                // 블러 효과가 있는 글로우
                                let glow_rect = resp_spec.rect.expand(8.0 * scale);
                                let glow_shape = egui::epaint::RectShape::filled(
                                    glow_rect,
                                    egui::CornerRadius::same((12.0 * scale) as u8),
                                    egui::Color32::from_rgba_premultiplied(35, 140, 240, 80),
                                ).with_blur_width(12.0 * scale);
                                ui.painter().add(glow_shape);
                            }
                            if resp_spec.clicked() {
                                println!("Selected: charge specific watts");
                                self.specific_watts_clicked = true;
                            }

                            ui.add_space(10.0 * scale);

                            let percent_btn = egui::Button::new(
                                egui::RichText::new("Charge by percent")
                                    .font(egui::FontId::proportional(16.0 * scale))
                                    .color(egui::Color32::WHITE),
                            )
                            .min_size(btn_size)
                            .fill(egui::Color32::from_rgba_premultiplied(160, 120, 240, instruction_alpha))
                            .corner_radius(egui::CornerRadius::same(10));

                            let resp_pct = ui.add(percent_btn);
                            if resp_pct.hovered() || resp_pct.is_pointer_button_down_on() {
                                // 블러 효과가 있는 글로우
                                let glow_rect = resp_pct.rect.expand(8.0 * scale);
                                let glow_shape = egui::epaint::RectShape::filled(
                                    glow_rect,
                                    egui::CornerRadius::same((12.0 * scale) as u8),
                                    egui::Color32::from_rgba_premultiplied(160, 120, 240, 80),
                                ).with_blur_width(12.0 * scale);
                                ui.painter().add(glow_shape);
                            }
                            if resp_pct.clicked() {
                                println!("Selected: charge by percent");
                                self.percent_clicked = true;
                            }

                            ui.add_space(10.0 * scale);

                            let full_btn = egui::Button::new(
                                egui::RichText::new("Full charge")
                                    .font(egui::FontId::proportional(16.0 * scale))
                                    .color(egui::Color32::WHITE),
                            )
                            .min_size(btn_size)
                            .fill(egui::Color32::from_rgba_premultiplied(20, 180, 120, instruction_alpha))
                            .corner_radius(egui::CornerRadius::same(10));

                            let resp_full = ui.add(full_btn);
                            if resp_full.hovered() || resp_full.is_pointer_button_down_on() {
                                // 블러 효과가 있는 글로우
                                let glow_rect = resp_full.rect.expand(8.0 * scale);
                                let glow_shape = egui::epaint::RectShape::filled(
                                    glow_rect,
                                    egui::CornerRadius::same((12.0 * scale) as u8),
                                    egui::Color32::from_rgba_premultiplied(20, 180, 120, 80),
                                ).with_blur_width(12.0 * scale);
                                ui.painter().add(glow_shape);
                            }
                            if resp_full.clicked() {
                                println!("Selected: full charge");
                                self.full_charge_clicked = true;
                            }
                        });
                    });
                }
            });

        // 60fps로 업데이트
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}
