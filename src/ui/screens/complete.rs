use crate::prelude::*;
use crate::ui::theme::Theme;

pub struct CompleteScreen {
    // 완료 화면에 특정한 상태가 있다면 여기에 추가합니다.
}

impl CompleteScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) -> bool {
        let mut back_to_home = false;
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Charging Complete")
                        .font(theme.fonts.heading.clone())
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
