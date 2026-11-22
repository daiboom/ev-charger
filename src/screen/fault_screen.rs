use eframe::egui;

use crate::layout::app_bar::show_app_bar;
use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::screen::standby_screen::panel_frame;

pub enum FaultAction {
    None,
    Acknowledge,
    ClearFault,
}

pub fn show(
    ctx: &egui::Context,
    message: &str,
    delivered_kwh: f64,
    cost: f64,
    currency: &str,
) -> FaultAction {
    let scale = calculate_scale(ctx);
    let mut action = FaultAction::None;
    if show_app_bar(ctx, scale, "Charging stopped", false) {
        action = FaultAction::Acknowledge;
    }

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(28.0 * scale);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(message)
                        .font(egui::FontId::proportional(26.0 * scale))
                        .color(egui::Color32::from_rgb(255, 140, 140)),
                );
                ui.add_space(12.0 * scale);
                ui.label(
                    egui::RichText::new("The contactor is open.")
                        .font(egui::FontId::proportional(16.0 * scale))
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(8.0 * scale);
                ui.label(
                    egui::RichText::new(format!(
                        "Delivered {delivered_kwh:.2} kWh · {cost:.0} {currency}"
                    ))
                    .font(egui::FontId::proportional(16.0 * scale))
                    .color(egui::Color32::from_gray(210)),
                );
                ui.add_space(20.0 * scale);
                if shadow_button(
                    ui,
                    "Return to standby",
                    egui::vec2(240.0 * scale, 52.0 * scale),
                    egui::Color32::from_rgb(20, 150, 110),
                    egui::Color32::WHITE,
                    scale,
                    10.0,
                )
                .clicked()
                {
                    action = FaultAction::Acknowledge;
                }
                ui.add_space(10.0 * scale);
                if shadow_button(
                    ui,
                    "Clear simulator fault",
                    egui::vec2(240.0 * scale, 42.0 * scale),
                    egui::Color32::from_rgb(55, 60, 80),
                    egui::Color32::WHITE,
                    scale,
                    8.0,
                )
                .clicked()
                {
                    action = FaultAction::ClearFault;
                }
            });
        });

    action
}
