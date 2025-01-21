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
        Box::new(|_| Ok(Box::new(demo::MyApp::default()))),
    )?;

    Ok(())
}
