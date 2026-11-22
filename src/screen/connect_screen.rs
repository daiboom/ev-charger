use eframe::egui;

use crate::layout::app_bar::show_app_bar;
use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::layout::stepper::stepper;
use crate::port::{ChargerSnapshot, ConnectionState};
use crate::screen::standby_screen::panel_frame;

pub struct ConnectActions {
    pub back: bool,
    pub plug: bool,
    pub ready: bool,
    pub unplug: bool,
    pub fault: bool,
    pub clear_fault: bool,
}

impl ConnectActions {
    fn none() -> Self {
        Self {
            back: false,
            plug: false,
            ready: false,
            unplug: false,
            fault: false,
            clear_fault: false,
        }
    }
}

pub fn show(ctx: &egui::Context, snapshot: &ChargerSnapshot) -> ConnectActions {
    let scale = calculate_scale(ctx);
    let mut actions = ConnectActions::none();
    actions.back = show_app_bar(ctx, scale, "Connect the vehicle", true);

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(8.0 * scale);
            stepper(
                ui,
                &["Amount", "Connect", "Payment", "Charge", "Done"],
                1,
                scale,
            );
            ui.add_space(24.0 * scale);
            ui.vertical_centered(|ui| {
                let (headline, detail, color) = connection_copy(snapshot);
                ui.label(
                    egui::RichText::new(headline)
                        .font(egui::FontId::proportional(30.0 * scale))
                        .color(color),
                );
                ui.add_space(8.0 * scale);
                ui.label(
                    egui::RichText::new(detail)
                        .font(egui::FontId::proportional(16.0 * scale))
                        .color(egui::Color32::from_gray(210)),
                );
                if let Some(soc) = snapshot.vehicle_soc_percent {
                    ui.add_space(8.0 * scale);
                    ui.label(
                        egui::RichText::new(format!("Reported battery {soc:.0}%"))
                            .font(egui::FontId::proportional(16.0 * scale))
                            .color(egui::Color32::WHITE),
                    );
                }
                ui.add_space(28.0 * scale);
                ui.label(
                    egui::RichText::new("Simulator controls")
                        .font(egui::FontId::proportional(14.0 * scale))
                        .color(egui::Color32::from_rgb(255, 196, 87)),
                );
                ui.add_space(8.0 * scale);
                ui.horizontal(|ui| {
                    actions.plug |= small(ui, scale, "Plug in");
                    actions.ready |= small(ui, scale, "Vehicle ready");
                    actions.unplug |= small(ui, scale, "Unplug");
                });
                ui.add_space(8.0 * scale);
                ui.horizontal(|ui| {
                    actions.fault |= small(ui, scale, "Fault");
                    actions.clear_fault |= small(ui, scale, "Clear fault");
                });
            });
        });

    actions
}

fn connection_copy(snapshot: &ChargerSnapshot) -> (&'static str, &'static str, egui::Color32) {
    if let Some(fault) = snapshot.fault {
        return (
            "Connection fault",
            fault.message(),
            egui::Color32::from_rgb(255, 120, 120),
        );
    }
    match snapshot.connection {
        ConnectionState::Disconnected => (
            "Connect the cable",
            "Waiting for a vehicle. Use the simulator when no charger is attached.",
            egui::Color32::WHITE,
        ),
        ConnectionState::Connected => (
            "Cable detected",
            "The vehicle has not released charging yet.",
            egui::Color32::from_rgb(255, 196, 87),
        ),
        ConnectionState::Ready => (
            "Vehicle ready",
            "Payment is next. The contactor stays open until payment is approved.",
            egui::Color32::from_rgb(120, 220, 140),
        ),
    }
}

fn small(ui: &mut egui::Ui, scale: f32, label: &str) -> bool {
    shadow_button(
        ui,
        label,
        egui::vec2(150.0 * scale, 42.0 * scale),
        egui::Color32::from_rgb(55, 60, 80),
        egui::Color32::WHITE,
        scale,
        8.0,
    )
    .clicked()
}
