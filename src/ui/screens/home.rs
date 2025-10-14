use crate::prelude::*;
use crate::ui::theme::Theme;
use crate::ui::components::styled_button;
use crate::app::AppState; // Keep AppState import for now, might be removed later if not needed

pub struct HomeScreen {
    // 홈 화면에 특정한 상태가 있다면 여기에 추가합니다.
}

impl HomeScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, theme: &Theme) -> bool { // Changed return type back to bool
        // 전체 화면을 채우도록 설정
        let mut start_charging = false; // Changed variable name and type back
        ui.centered_and_justified(|ui| {
            let panel_frame = egui::Frame {
                inner_margin: egui::Margin::same(theme.sizes.spacing as i8),
                outer_margin: egui::Margin::same(0 as i8),
                corner_radius: egui::epaint::CornerRadius::same(theme.sizes.border_radius as u8),
                shadow: egui::epaint::Shadow::NONE,
                fill: theme.colors.surface,
                stroke: egui::Stroke::new(1.0, theme.colors.secondary),
            };

            panel_frame.show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    // 충전기 상태 표시
                    ui.add(egui::Label::new(
                        egui::RichText::new("EV Charger Kiosk")
                            .font(theme.fonts.heading.clone())
                            .color(theme.colors.text_primary),
                    ));
                    ui.add_space(theme.sizes.spacing * 2.0); // More space
                    ui.add(egui::Label::new(
                        egui::RichText::new("Status: Available")
                            .font(egui::FontId::proportional(theme.fonts.heading.size * 0.8)) // Larger font
                            .color(theme.colors.success),
                    ));
                    ui.add_space(theme.sizes.spacing * 3.0); // More space

                    // 충전기 정보
                    self.charger_info(ui, theme);
                    ui.add_space(theme.sizes.spacing * 4.0); // More space

                    // 충전 시작 버튼
                    let start_charging_button_text = egui::RichText::new("Start Charging")
                        .font(theme.fonts.button.clone())
                        .color(theme.colors.text_primary);
                    let start_charging_button_widget = styled_button(start_charging_button_text)
                        .min_size(egui::vec2(350.0, theme.sizes.button_height * 1.2)); // Taller for kiosk
                    if ui.add(start_charging_button_widget).clicked()
                    {
                        info!("Start Charging button clicked");
                        start_charging = true; // Changed back to true
                    }
                });
            });
        });
        start_charging // Changed return value back to start_charging
    }

    fn charger_info(&self, ui: &mut egui::Ui, theme: &Theme) {
        egui::Grid::new("charger_info_grid")
            .num_columns(2)
            .spacing([theme.sizes.spacing * 2.0, theme.sizes.spacing])
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Charger ID:").color(theme.colors.text_secondary));
                ui.label(egui::RichText::new("EV-12345").color(theme.colors.text_primary));
                ui.end_row();

                ui.label(egui::RichText::new("Connector:").color(theme.colors.text_secondary));
                ui.label(egui::RichText::new("CCS Combo 2").color(theme.colors.text_primary));
                ui.end_row();

                ui.label(egui::RichText::new("Max Power:").color(theme.colors.text_secondary));
                ui.label(egui::RichText::new("150 kW").color(theme.colors.text_primary));
                ui.end_row();
            });
    }
}