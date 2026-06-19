mod core;
mod text;
mod trace;
mod renderer;
mod format;
mod i18n;
mod ui;
mod app;

use app::FlwingApp;
use log::info;

fn main() -> eframe::Result<()> {
    // 로거 초기화
    env_logger::init();
    info!("Starting Flwing...");

    let native_options = eframe::NativeOptions {
        // eframe 0.31에서는 viewport 설정 방식이 다를 수 있음. 기본값 사용.
        ..Default::default()
    };

    eframe::run_native(
        "flwing",
        native_options,
        Box::new(|cc| Ok(Box::new(FlwingApp::new(cc)))),
    )
}
