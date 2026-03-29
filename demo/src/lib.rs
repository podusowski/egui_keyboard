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
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            // Inject text events into Egui context. This function needs to be called before any widget is
            // created, otherwise the key presses will be ignored.
            self.keyboard.pump_events(ui.ctx());

            Window::new("Hello")
                .constrain_to(self.keyboard.safe_rect(ui.ctx()))
                .show(ui.ctx(), |ui| {
                    ui.label("This window is constrained to the \"safe\" area, which is not covered by the keyboard.");
                    ui.text_edit_singleline(&mut self.text);
                });

            // Show the keyboard.
            self.keyboard.show(ui.ctx());
        });
    }
}
