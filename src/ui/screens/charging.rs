use crate::prelude::*;
use crate::ui::theme::Theme;

pub struct ChargingScreen {
    // 충전 화면에 특정한 상태가 있다면 여기에 추가합니다.
}

impl ChargingScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) -> bool {
        let mut stop_charging = false;
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Charging...")
                        .font(theme.fonts.heading.clone())
                        .color(theme.colors.text_primary),
                );
                ui.add_space(theme.sizes.spacing * 2.0);
                // TODO: Add more charging information here

                let button = egui::Button::new(
                    egui::RichText::new("Stop Charging")
                        .font(theme.fonts.button.clone())
                        .color(theme.colors.text_primary),
                )
                .min_size(egui::vec2(250.0, theme.sizes.button_height));

                if ui.add(button).clicked() {
                    info!("Stop Charging button clicked");
                    stop_charging = true;
                }
            });
        });
        stop_charging
    }
}
