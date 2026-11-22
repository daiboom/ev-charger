use eframe::egui;

use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::layout::stepper::stepper;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetKind {
    Energy,
    Soc,
    Full,
}

pub fn show(ctx: &egui::Context) -> Option<TargetKind> {
    let scale = calculate_scale(ctx);
    let mut chosen = None;

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(12.0 * scale);
            stepper(
                ui,
                &["Amount", "Connect", "Payment", "Charge", "Done"],
                0,
                scale,
            );
            ui.add_space(28.0 * scale);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Choose a charge")
                        .font(egui::FontId::proportional(28.0 * scale))
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(8.0 * scale);
                ui.label(
                    egui::RichText::new(
                        "Energy, battery level, or a full charge. Speed comes next.",
                    )
                    .font(egui::FontId::proportional(16.0 * scale))
                    .color(egui::Color32::from_gray(200)),
                );
                ui.add_space(24.0 * scale);
                chosen = chosen.or(kind_button(
                    ui,
                    scale,
                    "Charge by energy (kWh)",
                    TargetKind::Energy,
                    egui::Color32::from_rgb(35, 140, 240),
                ));
                ui.add_space(10.0 * scale);
                chosen = chosen.or(kind_button(
                    ui,
                    scale,
                    "Charge to battery %",
                    TargetKind::Soc,
                    egui::Color32::from_rgb(120, 90, 200),
                ));
                ui.add_space(10.0 * scale);
                chosen = chosen.or(kind_button(
                    ui,
                    scale,
                    "Full charge",
                    TargetKind::Full,
                    egui::Color32::from_rgb(20, 150, 110),
                ));
            });
        });

    chosen
}

fn kind_button(
    ui: &mut egui::Ui,
    scale: f32,
    label: &str,
    kind: TargetKind,
    fill: egui::Color32,
) -> Option<TargetKind> {
    let response = shadow_button(
        ui,
        label,
        egui::vec2(280.0 * scale, 56.0 * scale),
        fill,
        egui::Color32::WHITE,
        scale,
        10.0,
    );
    response.clicked().then_some(kind)
}

pub(crate) fn panel_frame() -> egui::Frame {
    egui::Frame::NONE.fill(egui::Color32::from_rgb(20, 30, 50))
}
