mod app;

const APP_NAME: &str = "chanki";

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
