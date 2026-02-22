mod app;

use app::FuzzClip;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Fuzz Clip",
        options,
        Box::new(|_cc| Ok(Box::new(FuzzClip::new(_cc)))),
    )
}
