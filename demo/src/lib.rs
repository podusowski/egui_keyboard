use eframe::egui::{self, CentralPanel, Window};
use egui_keyboard::Keyboard;

pub struct MyApp {
    text: String,
    keyboard: Keyboard,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "edit me".to_string(),
            keyboard: Keyboard::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Inject text events into Egui context. This function needs to be called before any widget is
            // created, otherwise the key presses will be ignored.
            self.keyboard.pump_events(ctx);

            Window::new("Hello")
                .constrain_to(self.keyboard.safe_rect(ctx))
                .show(ui.ctx(), |ui| {
                    ui.label("This window is constrained to the \"safe\" area, which is not covered by the keyboard.");
                    ui.text_edit_singleline(&mut self.text);
                });

            // Show the keyboard.
            self.keyboard.show(ctx);
        });
    }
}
