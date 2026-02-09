use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "clip-hist",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
    )
}

struct MyApp {
    history: Vec<String>,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            history: vec!["hoge".to_string(), "huge".to_string()],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("clip-hist");
            for text in &self.history {
                ui.horizontal(|ui| ui.label(text));
            }
        });
    }
}
