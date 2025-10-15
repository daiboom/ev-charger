use eframe::egui;

pub fn shadow_button(
    ui: &mut egui::Ui,
    text: &str,
    min_size: egui::Vec2,
    fill: egui::Color32,
    text_color: egui::Color32,
    scale: f32,
    radius: f32,
) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text)
            .font(egui::FontId::proportional(16.0 * scale))
            .color(text_color),
    )
    .min_size(min_size)
    .fill(fill)
    .corner_radius(egui::CornerRadius::same(radius as u8));

    let resp = ui.add(button);

    if resp.hovered() || resp.is_pointer_button_down_on() {
        // soft glow with two layered rects
        let glow = 12.0 * scale;
        let rect_outer = resp.rect.expand(glow);
        let rect_inner = resp.rect.expand(glow * 0.6);
        let mut rgba = [fill.r(), fill.g(), fill.b(), 0u8];
        // outer
        rgba[3] = 70;
        let color_outer = egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
        // inner
        rgba[3] = 110;
        let color_inner = egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
        let r = radius as u8;
        ui.painter().rect_filled(
            rect_outer,
            egui::CornerRadius::same(r.saturating_add(4)),
            color_outer,
        );
        ui.painter().rect_filled(
            rect_inner,
            egui::CornerRadius::same(r.saturating_add(2)),
            color_inner,
        );
    }

    resp
}


