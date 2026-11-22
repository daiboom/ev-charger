use eframe::egui;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub use ev_charger::{domain, port, sim};

mod layout;
mod router;
mod screen;

use router::Router;

fn main() -> Result<(), eframe::Error> {
    init_tracing();
    let fullscreen = std::env::args().any(|arg| arg == "--fullscreen");
    info!(fullscreen, "starting ev charger simulator");

    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_min_inner_size([640.0, 480.0])
        .with_resizable(true)
        .with_decorations(!fullscreen)
        .with_fullscreen(fullscreen);

    eframe::run_native(
        "EV Charger",
        eframe::NativeOptions {
            viewport,
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(EvChargerApp::new()))),
    )
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

struct EvChargerApp {
    router: Router,
}

impl EvChargerApp {
    fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

impl eframe::App for EvChargerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        self.router.update(ctx, frame);
    }
}
