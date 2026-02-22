use eframe::egui;

pub struct FuzzClip {
    history: Vec<String>,
}

impl FuzzClip {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            history: vec!["hoge".to_string(), "huge".to_string()],
        }
    }
}

impl eframe::App for FuzzClip {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fuzz Clip");
            for text in &self.history {
                ui.label(text);
            }
        });
    }
}
