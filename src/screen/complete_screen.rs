use eframe::egui;

use crate::domain::{ChargeSummary, ChargeTarget};
use crate::layout::app_bar::show_app_bar;
use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::layout::stepper::stepper;
use crate::screen::charging_screen::format_duration;
use crate::screen::standby_screen::panel_frame;

pub fn show(ctx: &egui::Context, summary: &ChargeSummary) -> bool {
    let scale = calculate_scale(ctx);
    let mut home = show_app_bar(ctx, scale, "Complete", false);

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(8.0 * scale);
            stepper(
                ui,
                &["Amount", "Connect", "Payment", "Charge", "Done"],
                4,
                scale,
            );
            ui.add_space(18.0 * scale);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Charging complete")
                        .font(egui::FontId::proportional(28.0 * scale))
                        .color(egui::Color32::from_rgb(140, 230, 160)),
                );
                ui.add_space(14.0 * scale);
                line(ui, scale, &target_line(summary));
                line(
                    ui,
                    scale,
                    &format!("Delivered {:.2} kWh", summary.delivered_kwh),
                );
                if let Some(soc) = summary.soc_percent {
                    line(ui, scale, &format!("Battery {soc:.0}%"));
                }
                line(
                    ui,
                    scale,
                    &format!("Duration {}", format_duration(summary.duration)),
                );
                line(
                    ui,
                    scale,
                    &format!(
                        "Payment {} · {}",
                        summary.payment_method.label(),
                        summary.transaction_id
                    ),
                );
                ui.add_space(8.0 * scale);
                ui.label(
                    egui::RichText::new(format!("{:.0} {}", summary.cost, summary.currency))
                        .font(egui::FontId::proportional(26.0 * scale))
                        .color(egui::Color32::from_rgb(255, 196, 87)),
                );
                ui.add_space(6.0 * scale);
                line(ui, scale, &format!("Ended by {}", stop_label(summary)));
                ui.add_space(18.0 * scale);
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
                    home = true;
                }
                ui.add_space(8.0 * scale);
                if ui
                    .add(
                        egui::Button::new("Print receipt")
                            .min_size(egui::vec2(160.0 * scale, 36.0 * scale)),
                    )
                    .clicked()
                {
                    tracing::info!(
                        transaction_id = %summary.transaction_id,
                        cost = summary.cost,
                        "receipt requested in the simulator; no printer is attached"
                    );
                }
            });
        });

    home
}

fn target_line(summary: &ChargeSummary) -> String {
    let target = match summary.selection.target {
        ChargeTarget::Energy { kwh } => format!("{kwh:.0} kWh"),
        ChargeTarget::Soc { percent } => format!("{percent:.0}%"),
        ChargeTarget::Full => "Full".to_string(),
    };
    format!("Requested {target} · {}", summary.selection.speed.label())
}

fn stop_label(summary: &ChargeSummary) -> &'static str {
    match summary.stop_reason {
        crate::domain::StopReason::UserStop => "driver stop",
        crate::domain::StopReason::TargetReached => "target reached",
    }
}

fn line(ui: &mut egui::Ui, scale: f32, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .font(egui::FontId::proportional(16.0 * scale))
            .color(egui::Color32::WHITE),
    );
}
