use eframe::egui::{self, CentralPanel};
use egui_keyboard::Keyboard;

struct MyApp {
    text: String,
    keyboard: Keyboard,
}

impl MyApp {
    fn new() -> Self {
        Self {
            text: "edit me".to_string(),
            keyboard: Keyboard::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.keyboard.pump_events(ctx);
            ui.heading("egui keyboard demo");
            ui.text_edit_singleline(&mut self.text);
            self.keyboard.show(ctx);
        });
    }
}
