use egui::{Button, WidgetText};
use crate::ui::theme::Theme;

// A simple helper function to create a styled button
pub fn styled_button(text: impl Into<WidgetText>) -> Button<'static> {
    Button::new(text)
        .fill(egui::Color32::from_rgb(0, 120, 215)) // Use primary color from new theme
        .stroke(egui::Stroke::NONE)
        .corner_radius(12.0) // Use larger border_radius from new theme
}

// You can add more custom components or styling helpers here
// For example, a styled text input, or a custom container.

use crate::app::AppState;

pub fn global_navigation_panel(ui: &mut egui::Ui, theme: &Theme) -> Option<AppState> {
    let mut next_state: Option<AppState> = None;

    ui.vertical(|ui| {
        ui.add_space(theme.sizes.spacing);
        ui.label(egui::RichText::new("Global Navigation").font(theme.fonts.heading.clone()).color(theme.colors.text_primary));
        ui.add_space(theme.sizes.spacing);

        let home_button_text = egui::RichText::new("Home")
            .font(theme.fonts.button.clone())
            .color(theme.colors.text_primary);
        let home_button_widget = styled_button(home_button_text)
            .min_size(egui::vec2(180.0, theme.sizes.button_height)); // Reduced width
        if ui.add(home_button_widget).clicked()
        {
            next_state = Some(AppState::Home);
        }
        ui.add_space(theme.sizes.spacing);

        let charging_button_text = egui::RichText::new("Charging")
            .font(theme.fonts.button.clone())
            .color(theme.colors.text_primary);
        let charging_button_widget = styled_button(charging_button_text)
            .min_size(egui::vec2(180.0, theme.sizes.button_height)); // Reduced width
        if ui.add(charging_button_widget).clicked()
        {
            next_state = Some(AppState::Charging);
        }
        ui.add_space(theme.sizes.spacing);

        let payment_button_text = egui::RichText::new("Payment")
            .font(theme.fonts.button.clone())
            .color(theme.colors.text_primary);
        let payment_button_widget = styled_button(payment_button_text)
            .min_size(egui::vec2(180.0, theme.sizes.button_height)); // Reduced width
        if ui.add(payment_button_widget).clicked()
        {
            next_state = Some(AppState::Payment);
        }
        ui.add_space(theme.sizes.spacing);

        let complete_button_text = egui::RichText::new("Complete")
            .font(theme.fonts.button.clone())
            .color(theme.colors.text_primary);
        let complete_button_widget = styled_button(complete_button_text)
            .min_size(egui::vec2(180.0, theme.sizes.button_height)); // Reduced width
        if ui.add(complete_button_widget).clicked()
        {
            next_state = Some(AppState::Complete);
        }
        ui.add_space(theme.sizes.spacing);

        // Removed Error button from global navigation
        // let error_button_text = egui::RichText::new("Error")
        //     .font(theme.fonts.button.clone())
        //     .color(theme.colors.text_primary);
        // let error_button_widget = styled_button(error_button_text)
        //     .min_size(egui::vec2(180.0, theme.sizes.button_height));
        // if ui.add(error_button_widget).clicked()
        // {}
        // ui.add_space(theme.sizes.spacing);
    });

    next_state
}