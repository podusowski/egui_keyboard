use eframe::egui::{self, CentralPanel};
use egui_keyboard::Keyboard;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(
    app: egui_winit::winit::platform::android::activity::AndroidApp,
) -> Result<(), Box<dyn std::error::Error>> {
    use eframe::{NativeOptions, Renderer};

    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("egui_keyboard_demo")
            .with_max_level(log::LevelFilter::Debug),
    );
    let mut options = NativeOptions::default();
    options.renderer = Renderer::Wgpu;
    options.android_app = Some(app);
    eframe::run_native(
        "egui keyboard demo",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new()))),
    )?;

    Ok(())
}

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
