use eframe::egui::{self, CentralPanel, Window};
use egui_keyboard::Keyboard;

pub struct MyApp {
    text: String,
    text2: String,
    keyboard: Keyboard,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "edit me".to_string(),
            text2: "edit me".to_string(),
            keyboard: Keyboard::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.keyboard.pump_events(ctx);
            ui.heading("egui keyboard demo");
            ui.text_edit_singleline(&mut self.text);

            Window::new("Hello")
                .constrain_to(self.keyboard.safe_rect(ctx))
                .show(ui.ctx(), |ui| {
                    ui.label("This window is constrained to the \"safe\" area, which is not covered by the keyboard.");
                    ui.text_edit_singleline(&mut self.text2);
                });

            self.keyboard.show(ctx);
        });
    }
}
