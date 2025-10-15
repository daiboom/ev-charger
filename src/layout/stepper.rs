use eframe::egui;

pub fn stepper(ui: &mut egui::Ui, labels: &[&str], current: usize, scale: f32) {
    let dot_size = 18.0 * scale;
    let spacing = 16.0 * scale;
    let line_thickness = 3.0 * scale;

    ui.horizontal_centered(|ui| {
        for (idx, &label) in labels.iter().enumerate() {
            // Dot
            let is_active = idx == current;
            let color = if is_active {
                egui::Color32::from_rgb(0, 150, 255)
            } else {
                egui::Color32::from_gray(140)
            };
            let (rect, _resp) = ui.allocate_exact_size(egui::vec2(dot_size, dot_size), egui::Sense::hover());
            ui.painter().circle_filled(rect.center(), dot_size * 0.5, color);

            // Label
            ui.vertical(|ui| {
                ui.add_space(2.0 * scale);
                let text = egui::RichText::new(label)
                    .font(egui::FontId::proportional(14.0 * scale))
                    .color(if is_active { egui::Color32::WHITE } else { egui::Color32::from_gray(200) });
                ui.add(egui::Label::new(text));
            });

            // Line to next
            if idx < labels.len() - 1 {
                let (line_rect, _)
                    = ui.allocate_exact_size(egui::vec2(spacing, line_thickness), egui::Sense::hover());
                ui.painter().rect_filled(line_rect, 2.0, egui::Color32::from_gray(80));
            }
        }
    });
}


