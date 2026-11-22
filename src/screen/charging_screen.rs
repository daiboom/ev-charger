use std::time::Duration;

use eframe::egui;

use crate::domain::{ChargeTarget, LiveCharge, Selection};
use crate::layout::app_bar::show_app_bar;
use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::layout::stepper::stepper;
use crate::port::ChargerSnapshot;
use crate::screen::standby_screen::panel_frame;

pub enum ChargingAction {
    None,
    Stop,
    Unplug,
    Fault,
}

pub fn show(
    ctx: &egui::Context,
    selection: Selection,
    live: &LiveCharge,
    snapshot: &ChargerSnapshot,
    stopping: bool,
) -> ChargingAction {
    let scale = calculate_scale(ctx);
    let mut action = ChargingAction::None;
    let title = if stopping { "Stopping" } else { "Charging" };
    if show_app_bar(ctx, scale, title, false) {
        action = ChargingAction::Stop;
    }

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(8.0 * scale);
            stepper(
                ui,
                &["Amount", "Connect", "Payment", "Charge", "Done"],
                3,
                scale,
            );
            ui.add_space(16.0 * scale);
            ui.vertical_centered(|ui| {
                let fraction = progress_fraction(selection, live, snapshot);
                bar(ui, scale, fraction);
                ui.add_space(10.0 * scale);
                if let Some(soc) = live.soc_percent {
                    ui.label(
                        egui::RichText::new(format!("{soc:.0}%"))
                            .font(egui::FontId::proportional(28.0 * scale))
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.add_space(12.0 * scale);
                metric(
                    ui,
                    scale,
                    "Delivered",
                    &format!("{:.2} kWh", live.delivered_kwh),
                );
                metric(ui, scale, "Power", &format!("{:.1} kW", live.power_kw));
                metric(
                    ui,
                    scale,
                    "Meter",
                    &format!("{:.0} V · {:.1} A", live.voltage_v, live.current_a),
                );
                metric(ui, scale, "Elapsed", &format_duration(live.elapsed));
                metric(
                    ui,
                    scale,
                    "Cost",
                    &format!("{:.0} {}", live.cost, live.currency),
                );
                if let Some(eta) = eta(selection, live, snapshot) {
                    metric(ui, scale, "Remaining", &format_duration(eta));
                }
                ui.add_space(16.0 * scale);
                if !stopping
                    && shadow_button(
                        ui,
                        "Stop charging",
                        egui::vec2(240.0 * scale, 52.0 * scale),
                        egui::Color32::from_rgb(180, 70, 70),
                        egui::Color32::WHITE,
                        scale,
                        10.0,
                    )
                    .clicked()
                {
                    action = ChargingAction::Stop;
                }
                if !stopping {
                    ui.add_space(12.0 * scale);
                    ui.label(
                        egui::RichText::new("Simulator")
                            .font(egui::FontId::proportional(13.0 * scale))
                            .color(egui::Color32::from_rgb(255, 196, 87)),
                    );
                    ui.horizontal(|ui| {
                        if small(ui, scale, "Unplug").clicked() {
                            action = ChargingAction::Unplug;
                        }
                        if small(ui, scale, "Fault").clicked() {
                            action = ChargingAction::Fault;
                        }
                    });
                }
            });
        });

    action
}

fn progress_fraction(selection: Selection, live: &LiveCharge, snapshot: &ChargerSnapshot) -> f32 {
    match selection.target {
        ChargeTarget::Energy { kwh } if kwh > 0.0 => {
            (live.delivered_kwh as f32 / kwh).clamp(0.0, 1.0)
        }
        ChargeTarget::Soc { percent } if percent > 0.0 => {
            live.soc_percent.unwrap_or(0.0).clamp(0.0, percent) / percent
        }
        ChargeTarget::Full => live.soc_percent.unwrap_or(0.0) / 100.0,
        _ => snapshot.vehicle_soc_percent.unwrap_or(0.0) / 100.0,
    }
}

fn eta(selection: Selection, live: &LiveCharge, snapshot: &ChargerSnapshot) -> Option<Duration> {
    if live.power_kw < 0.2 {
        return None;
    }
    let hours = match selection.target {
        ChargeTarget::Energy { kwh } => {
            (f64::from(kwh) - live.delivered_kwh).max(0.0) / f64::from(live.power_kw)
        }
        ChargeTarget::Soc { percent } => {
            let capacity = snapshot.battery_capacity_kwh?;
            let soc = live.soc_percent?;
            f64::from((percent - soc).max(0.0) / 100.0 * capacity) / f64::from(live.power_kw)
        }
        ChargeTarget::Full => {
            let capacity = snapshot.battery_capacity_kwh?;
            let soc = live.soc_percent?;
            f64::from((100.0 - soc).max(0.0) / 100.0 * capacity) / f64::from(live.power_kw)
        }
    };
    if !hours.is_finite() {
        return None;
    }
    Some(Duration::from_secs_f64(hours * 3600.0))
}

fn bar(ui: &mut egui::Ui, scale: f32, fraction: f32) {
    let width = 420.0 * scale;
    let height = 22.0 * scale;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(8),
        egui::Color32::from_rgb(50, 55, 70),
    );
    let mut filled = rect;
    filled.max.x = rect.min.x + rect.width() * fraction.clamp(0.0, 1.0);
    ui.painter().rect_filled(
        filled,
        egui::CornerRadius::same(8),
        egui::Color32::from_rgb(80, 200, 130),
    );
}

fn metric(ui: &mut egui::Ui, scale: f32, label: &str, value: &str) {
    ui.add_space(4.0 * scale);
    ui.label(
        egui::RichText::new(format!("{label}: {value}"))
            .font(egui::FontId::proportional(18.0 * scale))
            .color(egui::Color32::WHITE),
    );
}

fn small(ui: &mut egui::Ui, scale: f32, label: &str) -> egui::Response {
    shadow_button(
        ui,
        label,
        egui::vec2(140.0 * scale, 40.0 * scale),
        egui::Color32::from_rgb(55, 60, 80),
        egui::Color32::WHITE,
        scale,
        8.0,
    )
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    format!("{}:{:02}", seconds / 60, seconds % 60)
}
