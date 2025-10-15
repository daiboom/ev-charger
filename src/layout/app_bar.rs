use eframe::egui;

pub struct AppBar {
    title: String,
    show_back_button: bool,
    back_clicked: bool,
}

impl AppBar {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            show_back_button: false,
            back_clicked: false,
        }
    }

    pub fn with_back_button(mut self) -> Self {
        self.show_back_button = true;
        self
    }

    pub fn is_back_clicked(&self) -> bool {
        self.back_clicked
    }

    pub fn reset_back_clicked(&mut self) {
        self.back_clicked = false;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, scale: f32) {
        egui::TopBottomPanel::top("app_bar")
            .frame(egui::Frame::default().fill(egui::Color32::from_rgba_premultiplied(20, 20, 25, 200)))
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(15.0 * scale, 0.0);

                    // Back button
                    if self.show_back_button {
                        let back_btn = egui::Button::new(
                            egui::RichText::new("←")
                                .font(egui::FontId::proportional(20.0 * scale))
                                .color(egui::Color32::WHITE),
                        )
                        .min_size(egui::vec2(40.0 * scale, 35.0 * scale))
                        .fill(egui::Color32::from_rgba_premultiplied(60, 60, 80, 255))
                        .corner_radius(egui::CornerRadius::same(8));

                        let resp = ui.add(back_btn);
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
                            self.back_clicked = true;
                        }
                    }

                    // Title
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.0 * scale);
                        ui.add(egui::Label::new(
                            egui::RichText::new(&self.title)
                                .font(egui::FontId::proportional(20.0 * scale))
                                .color(egui::Color32::WHITE),
                        ));
                        ui.add_space(5.0 * scale);
                    });

                    // Right space (for balance)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if !self.show_back_button {
                            ui.add_space(40.0 * scale);
                        }
                    });
                });

                ui.add_space(5.0 * scale);
            });
    }
}
