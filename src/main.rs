use eframe::egui;

mod screen;
mod layout;
mod router;
use router::Router;

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

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(true)
            .with_decorations(true)
            .with_always_on_top()
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "EV Charger",
        options,
        Box::new(|_cc| Ok(Box::new(EvChargerApp::new()))),
    )
}

impl eframe::App for EvChargerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.router.update(ctx, frame);
    }
}
