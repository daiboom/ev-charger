//! 테마 및 스타일 관리

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: ColorPalette,
    pub sizes: SizePalette,
    pub fonts: FontPalette,
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub primary: egui::Color32,
    pub secondary: egui::Color32,
    pub success: egui::Color32,
    pub warning: egui::Color32,
    pub error: egui::Color32,
    pub background: egui::Color32,
    pub surface: egui::Color32,
    pub text_primary: egui::Color32,
    pub text_secondary: egui::Color32,
}

#[derive(Debug, Clone)]
pub struct SizePalette {
    pub button_height: f32,
    pub input_height: f32,
    pub spacing: f32,
    pub border_radius: f32,
}

#[derive(Debug, Clone)]
pub struct FontPalette {
    pub heading: egui::FontId,
    pub body: egui::FontId,
    pub button: egui::FontId,
    pub small: egui::FontId,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            colors: ColorPalette {
                primary: egui::Color32::from_rgb(0, 120, 215), // More standard blue for light background
                secondary: egui::Color32::from_rgb(100, 100, 100), // Darker Gray for borders/less prominent elements
                success: egui::Color32::from_rgb(0, 150, 0), // Vibrant Green
                warning: egui::Color32::from_rgb(200, 100, 0), // Vibrant Orange
                error: egui::Color32::from_rgb(200, 0, 0), // Vibrant Red
                background: egui::Color32::from_rgb(230, 230, 235), // Very light gray/off-white
                surface: egui::Color32::from_rgb(255, 255, 255), // Pure white
                text_primary: egui::Color32::from_rgb(30, 30, 30), // Dark gray for text
                text_secondary: egui::Color32::from_rgb(80, 80, 80), // Medium gray for secondary text
            },
            sizes: SizePalette {
                button_height: 60.0, // Larger for kiosk
                input_height: 50.0,
                spacing: 12.0, // Reduced spacing
                border_radius: 15.0, // Slightly more rounded
            },
            fonts: FontPalette {
                heading: egui::FontId::proportional(32.0), // Larger for kiosk
                body: egui::FontId::proportional(20.0),
                button: egui::FontId::proportional(24.0),
                small: egui::FontId::proportional(16.0),
            },
        }
    }
    
    pub fn apply_to_context(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = self.colors.surface;
        visuals.widgets.inactive.bg_fill = self.colors.surface;
        visuals.widgets.hovered.bg_fill = self.colors.primary; // Use primary for hovered
        visuals.widgets.active.bg_fill = self.colors.primary; // Use primary for active
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.colors.secondary); // Subtle border
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.colors.secondary); // Subtle border
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, self.colors.primary); // Primary border on hover
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, self.colors.primary); // Primary border on active
        visuals.selection.bg_fill = self.colors.primary; // Selection background
        visuals.selection.stroke = egui::Stroke::new(1.0, self.colors.primary); // Selection border
        visuals.window_stroke = egui::Stroke::new(1.0, self.colors.secondary); // Window border color
        visuals.panel_fill = self.colors.background; // Panel background
        visuals.hyperlink_color = self.colors.primary; // Hyperlink color
        
        // Use default shadows for now, or set them explicitly if a valid API exists
        // visuals.window_shadow = egui::epaint::Shadow::small_dark(); // Removed
        // visuals.popup_shadow = egui::epaint::Shadow::small_dark(); // Removed

        // Rounding is applied per-widget or via egui::Style, not directly on WidgetVisuals.rounding field
        // visuals.widgets.noninteractive.rounding = egui::Rounding::same(self.sizes.border_radius); // Removed
        // visuals.widgets.inactive.rounding = egui::Rounding::same(self.sizes.border_radius); // Removed
        // visuals.widgets.hovered.rounding = egui::Rounding::same(self.sizes.border_radius); // Removed
        // visuals.widgets.active.rounding = egui::Rounding::same(self.sizes.border_radius); // Removed

        ctx.set_visuals(visuals);
    }
}
