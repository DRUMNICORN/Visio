use env_logger::Builder;
use log::{debug, error, LevelFilter};
use nodium_app::NodiumApp;
use nodium_events::NodiumEventBus;
use nodium_plugins::NodiumPlugins;
use nodium_tauri::NodiumViewTauri;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug) // Change this to the desired log level
        .init();
    println!("Welcome to Nodium!\n");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            tokio::spawn(async move {
                let tauri_view = NodiumViewTauri::new(handle.clone());

                let app = NodiumApp::init(Box::new(tauri_view)).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
