//! UI 모듈

pub mod components;
pub mod screens;
pub mod theme;

use crate::prelude::*;
use std::sync::Arc;

pub fn setup_fonts(ctx: &egui::Context, _config: &crate::config::UiConfig) {
    let mut fonts = egui::FontDefinitions::default();
    
    // 한글 폰트 추가 (macOS)
    fonts.font_data.insert(
        "korean_font".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "/System/Library/Fonts/AppleSDGothicNeo.ttc"
        ))),
    );
    
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "korean_font".to_owned());
    
    ctx.set_fonts(fonts);
}
