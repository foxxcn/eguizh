//! Demo app for egui
#![allow(clippy::missing_errors_doc)]

mod apps;
mod backend_panel;
mod frame_history;
mod wrap_app;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

pub use wrap_app::{Anchor, WrapApp};

/// Time of day as seconds since midnight. Used for clock in demo app.
pub(crate) fn seconds_since_midnight() -> f64 {
    use chrono::Timelike as _;
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;

// ----------------------------------------------------------------------------
// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
#[allow(unsafe_code)]
pub fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("egui_demo_app"),
    );

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1024.0])
            .with_drag_and_drop(true),
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "egui demo app",
        options,
        Box::new(|cc| Ok(Box::new(crate::WrapApp::new(cc)))),
    )
    .unwrap();
}
