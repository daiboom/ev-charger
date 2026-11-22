use eframe::egui;

use crate::domain::{PaymentMethod, Selection};
use crate::layout::app_bar::show_app_bar;
use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::layout::stepper::stepper;
use crate::port::PaymentStatus;
use crate::screen::standby_screen::panel_frame;

pub struct PaymentView<'a> {
    pub selection: Selection,
    pub estimate: Option<f64>,
    pub currency: &'a str,
    pub selected: Option<PaymentMethod>,
    pub status: PaymentStatus,
}

pub enum PaymentAction {
    None,
    Back,
    Choose(PaymentMethod),
    Start(PaymentMethod),
}

pub fn show(ctx: &egui::Context, view: PaymentView<'_>) -> PaymentAction {
    let scale = calculate_scale(ctx);
    let mut action = PaymentAction::None;
    if show_app_bar(ctx, scale, "Payment", true) {
        action = PaymentAction::Back;
    }
    let pending = matches!(&view.status, PaymentStatus::Pending { .. });

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(8.0 * scale);
            stepper(
                ui,
                &["Amount", "Connect", "Payment", "Charge", "Done"],
                2,
                scale,
            );
            ui.add_space(12.0 * scale);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(selection_line(view.selection))
                        .font(egui::FontId::proportional(18.0 * scale))
                        .color(egui::Color32::from_gray(210)),
                );
                ui.add_space(6.0 * scale);
                let estimate = match view.estimate {
                    Some(amount) => format!("Estimated {:.0} {}", amount, view.currency),
                    None => "The final amount follows the meter".to_string(),
                };
                ui.label(
                    egui::RichText::new(estimate)
                        .font(egui::FontId::proportional(20.0 * scale))
                        .color(egui::Color32::from_rgb(255, 196, 87)),
                );
                ui.add_space(8.0 * scale);
                ui.label(
                    egui::RichText::new(
                        "Simulator approval only. No card, phone, or bank is contacted.",
                    )
                    .font(egui::FontId::proportional(14.0 * scale))
                    .color(egui::Color32::from_gray(180)),
                );
                ui.add_space(16.0 * scale);

                for method in PaymentMethod::all() {
                    let selected = view.selected == Some(method);
                    let fill = if selected {
                        egui::Color32::from_rgb(20, 150, 110)
                    } else {
                        egui::Color32::from_rgb(55, 60, 80)
                    };
                    if shadow_button(
                        ui,
                        method.label(),
                        egui::vec2(320.0 * scale, 48.0 * scale),
                        fill,
                        egui::Color32::WHITE,
                        scale,
                        8.0,
                    )
                    .clicked()
                        && !pending
                    {
                        action = PaymentAction::Choose(method);
                    }
                    ui.add_space(8.0 * scale);
                }

                ui.add_space(8.0 * scale);
                match &view.status {
                    PaymentStatus::Pending { .. } => {
                        ui.label(
                            egui::RichText::new("Approving…")
                                .font(egui::FontId::proportional(18.0 * scale))
                                .color(egui::Color32::from_rgb(120, 190, 255)),
                        );
                    }
                    PaymentStatus::Declined { reason, .. } => {
                        ui.label(
                            egui::RichText::new(reason)
                                .font(egui::FontId::proportional(16.0 * scale))
                                .color(egui::Color32::from_rgb(255, 140, 140)),
                        );
                    }
                    PaymentStatus::Approved { transaction_id, .. } => {
                        ui.label(
                            egui::RichText::new(format!("Approved {transaction_id}"))
                                .font(egui::FontId::proportional(16.0 * scale))
                                .color(egui::Color32::from_rgb(120, 220, 140)),
                        );
                    }
                    PaymentStatus::Idle => {}
                }

                ui.add_space(12.0 * scale);
                if let Some(method) = view.selected {
                    if shadow_button(
                        ui,
                        "Start charging",
                        egui::vec2(240.0 * scale, 52.0 * scale),
                        if pending {
                            egui::Color32::from_gray(90)
                        } else {
                            egui::Color32::from_rgb(20, 150, 110)
                        },
                        egui::Color32::WHITE,
                        scale,
                        10.0,
                    )
                    .clicked()
                        && !pending
                    {
                        action = PaymentAction::Start(method);
                    }
                }
            });
        });

    action
}

fn selection_line(selection: Selection) -> String {
    let target = match selection.target {
        crate::domain::ChargeTarget::Energy { kwh } => format!("{kwh:.0} kWh"),
        crate::domain::ChargeTarget::Soc { percent } => format!("{percent:.0}%"),
        crate::domain::ChargeTarget::Full => "Full".to_string(),
    };
    format!("{target} · {}", selection.speed.label())
}
