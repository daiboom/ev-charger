use eframe::egui;

pub fn show_top_bar(ctx: &egui::Context, scale: f32, status: &str) {
    egui::TopBottomPanel::top("top_bar")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgba_premultiplied(20, 20, 25, 230)))
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(12.0, 0.0);
                ui.label(
                    egui::RichText::new("EV CHARGER")
                        .font(egui::FontId::proportional(14.0 * scale))
                        .color(egui::Color32::WHITE),
                );
                ui.separator();
                let now = chrono::Local::now();
                ui.label(
                    egui::RichText::new(now.format("%Y-%m-%d %H:%M:%S").to_string())
                        .font(egui::FontId::proportional(14.0 * scale))
                        .color(egui::Color32::from_gray(210)),
                );
                ui.label(
                    egui::RichText::new(status)
                        .font(egui::FontId::proportional(12.0 * scale))
                        .color(egui::Color32::from_rgb(255, 196, 87)),
                );
            });
        });
}
