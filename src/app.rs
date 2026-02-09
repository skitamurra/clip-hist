use eframe::egui;

pub struct ClipHist {
    history: Vec<String>,
}

impl ClipHist {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            history: vec!["hoge".to_string(), "huge".to_string()],
        }
    }
}

impl eframe::App for ClipHist {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("clip-hist");
            for text in &self.history {
                ui.label(text);
            }
        });
    }
}
