// nodium/src/main.rs
use nodium_egui::NodiumApp;
use eframe::run_native;

#[tokio::main]
async fn main() {
    let app = NodiumApp::default();
    let options = eframe::NativeOptions::default();
    run_native(Box::new(app), options);
}
