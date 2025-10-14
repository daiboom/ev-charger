use crate::prelude::*;
use crate::ui::theme::Theme;

pub struct ErrorScreen {
    message: String,
}

impl ErrorScreen {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) -> bool {
        let mut back_to_home = false;
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Error")
                        .font(theme.fonts.heading.clone())
                        .color(theme.colors.error),
                );
                ui.add_space(theme.sizes.spacing * 2.0);
                ui.label(
                    egui::RichText::new(&self.message)
                        .font(theme.fonts.body.clone())
                        .color(theme.colors.text_primary),
                );
                ui.add_space(theme.sizes.spacing * 2.0);

                let button = egui::Button::new(
                    egui::RichText::new("Back to Home")
                        .font(theme.fonts.button.clone())
                        .color(theme.colors.text_primary),
                )
                .min_size(egui::vec2(250.0, theme.sizes.button_height));

                if ui.add(button).clicked() {
                    info!("Back to Home button clicked");
                    back_to_home = true;
                }
            });
        });
        back_to_home
    }
}
