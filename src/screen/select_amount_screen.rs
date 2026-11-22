use eframe::egui;

use crate::domain::{ChargeSpeed, ChargeTarget};
use crate::layout::app_bar::show_app_bar;
use crate::layout::app_container::calculate_scale;
use crate::layout::shadow_button::shadow_button;
use crate::layout::stepper::stepper;
use crate::screen::standby_screen::{panel_frame, TargetKind};

pub struct AmountDraft {
    pub kind: TargetKind,
    pub amount: String,
    pub speed: ChargeSpeed,
    pub error: Option<String>,
}

impl AmountDraft {
    pub fn new(kind: TargetKind) -> Self {
        let amount = match kind {
            TargetKind::Energy => "20".to_string(),
            TargetKind::Soc => "80".to_string(),
            TargetKind::Full => String::new(),
        };
        Self {
            kind,
            amount,
            speed: ChargeSpeed::Normal,
            error: None,
        }
    }
}

pub enum SelectAction {
    None,
    Back,
    Proceed(ChargeTarget, ChargeSpeed),
}

pub fn show(ctx: &egui::Context, draft: &mut AmountDraft) -> SelectAction {
    let scale = calculate_scale(ctx);
    let mut action = SelectAction::None;
    if show_app_bar(ctx, scale, title(draft.kind), true) {
        action = SelectAction::Back;
    }

    egui::CentralPanel::default()
        .frame(panel_frame())
        .show(ctx, |ui| {
            ui.add_space(8.0 * scale);
            stepper(
                ui,
                &["Amount", "Connect", "Payment", "Charge", "Done"],
                0,
                scale,
            );
            ui.add_space(16.0 * scale);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(heading(draft.kind))
                        .font(egui::FontId::proportional(26.0 * scale))
                        .color(egui::Color32::WHITE),
                );
                ui.add_space(16.0 * scale);

                if draft.kind != TargetKind::Full {
                    amount_editor(ui, scale, draft);
                    ui.add_space(12.0 * scale);
                    presets(ui, scale, draft);
                    ui.add_space(18.0 * scale);
                }

                ui.label(
                    egui::RichText::new("Speed")
                        .font(egui::FontId::proportional(16.0 * scale))
                        .color(egui::Color32::from_gray(210)),
                );
                ui.add_space(8.0 * scale);
                speed_row(ui, scale, draft);
                ui.add_space(12.0 * scale);
                if let Some(error) = &draft.error {
                    ui.label(
                        egui::RichText::new(error)
                            .color(egui::Color32::from_rgb(255, 140, 140))
                            .font(egui::FontId::proportional(15.0 * scale)),
                    );
                }
                ui.add_space(16.0 * scale);
                let response = shadow_button(
                    ui,
                    "Continue",
                    egui::vec2(240.0 * scale, 52.0 * scale),
                    egui::Color32::from_rgb(20, 150, 110),
                    egui::Color32::WHITE,
                    scale,
                    10.0,
                );
                if response.clicked() {
                    match parse_target(draft.kind, &draft.amount) {
                        Ok(target) => action = SelectAction::Proceed(target, draft.speed),
                        Err(error) => draft.error = Some(error),
                    }
                }
            });
        });

    action
}

pub fn parse_target(kind: TargetKind, amount: &str) -> Result<ChargeTarget, String> {
    match kind {
        TargetKind::Full => Ok(ChargeTarget::Full),
        TargetKind::Energy => {
            let kwh: f32 = amount
                .trim()
                .parse()
                .map_err(|_| "Enter the energy in kWh.".to_string())?;
            if kwh.is_finite() && kwh > 0.0 && kwh <= 200.0 {
                Ok(ChargeTarget::Energy { kwh })
            } else {
                Err("Energy must be between 0 and 200 kWh.".to_string())
            }
        }
        TargetKind::Soc => {
            let percent: f32 = amount
                .trim()
                .parse()
                .map_err(|_| "Enter the target battery percent.".to_string())?;
            if percent.is_finite() && percent > 0.0 && percent <= 100.0 {
                Ok(ChargeTarget::Soc { percent })
            } else {
                Err("Battery target must be between 0 and 100%.".to_string())
            }
        }
    }
}

fn title(kind: TargetKind) -> &'static str {
    match kind {
        TargetKind::Energy => "Energy",
        TargetKind::Soc => "Battery level",
        TargetKind::Full => "Full charge",
    }
}

fn heading(kind: TargetKind) -> &'static str {
    match kind {
        TargetKind::Energy => "How much energy?",
        TargetKind::Soc => "Target battery level",
        TargetKind::Full => "Charge until full",
    }
}

fn amount_editor(ui: &mut egui::Ui, scale: f32, draft: &mut AmountDraft) {
    ui.horizontal(|ui| {
        if ui.add(step_button(scale, "−")).clicked() {
            nudge(draft, -step_of(draft.kind));
        }
        let width = 160.0 * scale;
        ui.add_sized(
            [width, 48.0 * scale],
            egui::TextEdit::singleline(&mut draft.amount)
                .font(egui::FontId::proportional(24.0 * scale))
                .desired_width(width)
                .horizontal_align(egui::Align::Center),
        );
        if ui.add(step_button(scale, "+")).clicked() {
            nudge(draft, step_of(draft.kind));
        }
        ui.label(
            egui::RichText::new(unit(draft.kind))
                .font(egui::FontId::proportional(20.0 * scale))
                .color(egui::Color32::WHITE),
        );
    });
}

fn presets(ui: &mut egui::Ui, scale: f32, draft: &mut AmountDraft) {
    let options: &[(&str, &str)] = match draft.kind {
        TargetKind::Energy => &[
            ("10 kWh", "10"),
            ("20 kWh", "20"),
            ("40 kWh", "40"),
            ("60 kWh", "60"),
        ],
        TargetKind::Soc => &[("50%", "50"), ("80%", "80"), ("90%", "90"), ("100%", "100")],
        TargetKind::Full => &[],
    };
    ui.horizontal(|ui| {
        for (label, value) in options {
            if ui
                .add(egui::Button::new(*label).min_size(egui::vec2(80.0 * scale, 36.0 * scale)))
                .clicked()
            {
                draft.amount = (*value).to_string();
                draft.error = None;
            }
        }
    });
}

fn speed_row(ui: &mut egui::Ui, scale: f32, draft: &mut AmountDraft) {
    ui.horizontal(|ui| {
        for speed in [ChargeSpeed::Slow, ChargeSpeed::Normal, ChargeSpeed::Fast] {
            let selected = draft.speed == speed;
            let fill = if selected {
                egui::Color32::from_rgb(35, 140, 240)
            } else {
                egui::Color32::from_rgb(55, 60, 80)
            };
            if shadow_button(
                ui,
                speed.label(),
                egui::vec2(110.0 * scale, 42.0 * scale),
                fill,
                egui::Color32::WHITE,
                scale,
                8.0,
            )
            .clicked()
            {
                draft.speed = speed;
            }
        }
    });
}

fn nudge(draft: &mut AmountDraft, delta: f32) {
    let current: f32 = draft.amount.trim().parse().unwrap_or(0.0);
    let next = match draft.kind {
        TargetKind::Energy => (current + delta).clamp(5.0, 200.0),
        TargetKind::Soc => (current + delta).clamp(5.0, 100.0),
        TargetKind::Full => current,
    };
    draft.amount = format!("{next:.0}");
    draft.error = None;
}

fn step_of(kind: TargetKind) -> f32 {
    match kind {
        TargetKind::Energy | TargetKind::Soc => 5.0,
        TargetKind::Full => 0.0,
    }
}

fn unit(kind: TargetKind) -> &'static str {
    match kind {
        TargetKind::Energy => "kWh",
        TargetKind::Soc => "%",
        TargetKind::Full => "",
    }
}

fn step_button(scale: f32, label: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(label)
            .font(egui::FontId::proportional(22.0 * scale))
            .color(egui::Color32::WHITE),
    )
    .min_size(egui::vec2(48.0 * scale, 48.0 * scale))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_energy_soc_and_full() {
        assert_eq!(
            parse_target(TargetKind::Energy, "12.5").unwrap(),
            ChargeTarget::Energy { kwh: 12.5 }
        );
        assert_eq!(
            parse_target(TargetKind::Soc, "80").unwrap(),
            ChargeTarget::Soc { percent: 80.0 }
        );
        assert_eq!(
            parse_target(TargetKind::Full, "").unwrap(),
            ChargeTarget::Full
        );
        assert!(parse_target(TargetKind::Energy, "0").is_err());
        assert!(parse_target(TargetKind::Soc, "120").is_err());
        assert!(parse_target(TargetKind::Energy, "kwh").is_err());
    }
}
