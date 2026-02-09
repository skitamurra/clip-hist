mod app;

use app::ClipHist;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "clipHist",
        options,
        Box::new(|_cc| Ok(Box::new(ClipHist::new(_cc)))),
    )
}
