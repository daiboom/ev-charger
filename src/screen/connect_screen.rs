use eframe::egui;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::layout::top_bar::show_top_bar;
// use crate::layout::app_bar::AppBar;
use crate::layout::app_container::calculate_scale;

pub struct ConnectScreen {
    start_time: Instant,
    connection_status: ConnectionStatus,
    background_image_path: Option<PathBuf>,
    background_image: Option<egui::TextureHandle>,
    proceed_clicked: bool,
    // app_bar: AppBar,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Waiting,      // Waiting for EV connection
    Connecting,   // Connecting to EV
    Verifying,    // Verifying communication protocol
    Finalizing,   // Finalizing connection
    Connected,    // EV connected
    Error,        // Connection error
}

impl ConnectScreen {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            connection_status: ConnectionStatus::Waiting,
            background_image_path: None,
            background_image: None,
            proceed_clicked: false,
            // app_bar: AppBar::new("Connect Your EV"),
        }
    }

    pub fn with_background_image(mut self, image_path: PathBuf) -> Self {
        self.background_image_path = Some(image_path);
        self
    }

    pub fn is_proceed_clicked(&self) -> bool {
        self.proceed_clicked
    }

    pub fn reset_proceed_clicked(&mut self) {
        self.proceed_clicked = false;
    }

    // pub fn is_back_clicked(&self) -> bool {
        // self.app_bar.is_back_clicked()
    // }

    // pub fn reset_back_clicked(&mut self) {
    //     self.app_bar.reset_back_clicked();
    // }

    pub fn get_connection_status(&self) -> &ConnectionStatus {
        &self.connection_status
    }

    pub fn is_connection_complete(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected)
    }

    fn load_background_image(&mut self, ctx: &egui::Context) {
        if let Some(ref path) = self.background_image_path {
            if self.background_image.is_none() {
                if let Ok(image_data) = std::fs::read(path) {
                    if let Ok(image) = image::load_from_memory(&image_data) {
                        let rgba_image = image.to_rgba8();
                        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                        let pixels = rgba_image.into_raw();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                        self.background_image = Some(ctx.load_texture("connect_bg", color_image, Default::default()));
                    }
                }
            }
        }
    }

    fn update_connection_status(&mut self) {
        let elapsed = self.start_time.elapsed();
        
        // Simulate EV charger connection detection
        match self.connection_status {
            ConnectionStatus::Waiting => {
                // Step 1: Connector detection (1 second)
                if elapsed > Duration::from_secs(1) {
                    self.connection_status = ConnectionStatus::Connecting;
                }
            }
            ConnectionStatus::Connecting => {
                // Step 2: Electrical connection verification (2 seconds)
                if elapsed > Duration::from_secs(2) {
                    self.connection_status = ConnectionStatus::Verifying;
                }
            }
            ConnectionStatus::Verifying => {
                // Step 3: Communication protocol verification (3 seconds)
                if elapsed > Duration::from_secs(3) {
                    self.connection_status = ConnectionStatus::Finalizing;
                }
            }
            ConnectionStatus::Finalizing => {
                // Step 4: Final connection completion (5 seconds)
                if elapsed > Duration::from_secs(5) {
                    self.connection_status = ConnectionStatus::Connected;
                }
            }
            ConnectionStatus::Connected => {
                // Connection maintained - continuous monitoring in real implementation
            }
            ConnectionStatus::Error => {
                // Error state - retry logic needed
            }
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let scale = calculate_scale(ctx);
        
        // Update connection status
        self.update_connection_status();

        show_top_bar(ctx, scale, None);

        // AppBar display
        // egui::CentralPanel::default()
        //     .frame(egui::Frame::NONE)
        //     .show(ctx, |ui| {
        //         self.app_bar.show(ui, scale);
        //     });

        // Main content - display different UI based on connection status
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // Layout for precise center alignment
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // Space for vertical center alignment
                    ui.add_space(ui.available_height() / 2.0 - 40.0 * scale);
                    
                    // Display message based on connection status
                    match self.connection_status {
                        ConnectionStatus::Waiting => {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Connect Your Car")
                                    .font(egui::FontId::proportional(32.0 * scale))
                                    .color(egui::Color32::WHITE),
                            ));
                            ui.add_space(10.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new("Please connect the charging cable")
                                    .font(egui::FontId::proportional(18.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)),
                            ));
                        }
                        ConnectionStatus::Connecting => {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Connecting...")
                                    .font(egui::FontId::proportional(32.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(100, 200, 255, 255)),
                            ));
                            ui.add_space(10.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new("Detecting vehicle connection")
                                    .font(egui::FontId::proportional(18.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)),
                            ));
                        }
                        ConnectionStatus::Verifying => {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Verifying...")
                                    .font(egui::FontId::proportional(32.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(255, 200, 100, 255)),
                            ));
                            ui.add_space(10.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new("Checking electrical connection")
                                    .font(egui::FontId::proportional(18.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)),
                            ));
                        }
                        ConnectionStatus::Finalizing => {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Finalizing...")
                                    .font(egui::FontId::proportional(32.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(255, 150, 100, 255)),
                            ));
                            ui.add_space(10.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new("Establishing communication protocol")
                                    .font(egui::FontId::proportional(18.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)),
                            ));
                        }
                        ConnectionStatus::Connected => {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Connected!")
                                    .font(egui::FontId::proportional(32.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(100, 255, 100, 255)),
                            ));
                            ui.add_space(10.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new("Vehicle ready for charging")
                                    .font(egui::FontId::proportional(18.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)),
                            ));
                        }
                        ConnectionStatus::Error => {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Connection Error")
                                    .font(egui::FontId::proportional(32.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(255, 100, 100, 255)),
                            ));
                            ui.add_space(10.0 * scale);
                            ui.add(egui::Label::new(
                                egui::RichText::new("Please check the connection")
                                    .font(egui::FontId::proportional(18.0 * scale))
                                    .color(egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)),
                            ));
                        }
                    }
                });
            });
    }
}
