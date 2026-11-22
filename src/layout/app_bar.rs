use eframe::egui;

/// Returns whether the back button was clicked this frame.
pub fn show_app_bar(ctx: &egui::Context, scale: f32, title: &str, show_back: bool) -> bool {
    let mut back_clicked = false;
    egui::TopBottomPanel::top("app_bar")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgba_premultiplied(20, 20, 25, 220)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(15.0 * scale, 0.0);
                if show_back {
                    let back_btn = egui::Button::new(
                        egui::RichText::new("Back")
                            .font(egui::FontId::proportional(16.0 * scale))
                            .color(egui::Color32::WHITE),
                    )
                    .min_size(egui::vec2(72.0 * scale, 36.0 * scale));
                    if ui.add(back_btn).clicked() {
                        back_clicked = true;
                    }
                }
                ui.label(
                    egui::RichText::new(title)
                        .font(egui::FontId::proportional(20.0 * scale))
                        .color(egui::Color32::WHITE),
                );
            });
            ui.add_space(4.0 * scale);
        });
    back_clicked
}
