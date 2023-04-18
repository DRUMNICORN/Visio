use eframe::run_native;
use nodium_app::NodiumApp;
use nodium_events::EventBus;
use nodium_plugins::Plugins;
use tokio::runtime::Runtime;

use env_logger::Builder;
use log::{debug, info, LevelFilter};

fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug) // Change this to the desired log level
        .init();
    debug!("Logger initialized");
    // welcome
    println!("Welcome to Nodium!\n");

    info!("Creating runtime");
    let rt = Runtime::new().unwrap();
    debug!("Runtime created");
    rt.block_on(async {
        info!("Creating event bus");
        let event_bus = EventBus::new();
        debug!("Event bus created");

        info!("Creating plugins");
        let plugins = Plugins::new(event_bus.clone()).await;
        debug!("Plugins created");

        info!("Registering event handlers");
        plugins.lock().await.listen().await;
        debug!("Event handlers registered");

        plugins.lock().await.reload().await;

        info!("NodiumApp starting");
        let app = NodiumApp::new(event_bus);
        debug!("NodiumApp created");

        let options = eframe::NativeOptions::default();
        run_native(Box::new(app), options);
    });
}
