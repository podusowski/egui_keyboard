use eframe::egui;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(
    app: egui_winit::winit::platform::android::activity::AndroidApp,
) -> Result<(), Box<dyn std::error::Error>> {
    use eframe::{NativeOptions, Renderer};
    use egui_winit::winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("egui-keyboard-demo")
            .with_max_level(log::LevelFilter::Info),
    );
    let mut options = NativeOptions::default();
    options.renderer = Renderer::Wgpu;
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));
    eframe::run_native("Walkers", options, Box::new(|_| Ok(Box::new(MyApp::new()))))?;

    Ok(())
}

struct MyApp;

impl MyApp {
    fn new() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
