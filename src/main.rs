use eframe::run_native;
use nodium_app::NodiumApp;
use nodium_events::EventBus;
use nodium_plugins::PluginManager;
use tokio::runtime::Runtime;

use env_logger::Builder;
use log::{debug, LevelFilter};

fn main() {
  Builder::new()
  .filter(None, LevelFilter::Debug) // Change this to the desired log level
  .init();

  let rt = Runtime::new().unwrap();
  debug!("Runtime created");


  rt.block_on(async {
      let event_bus = EventBus::new();
      let installer = PluginManager::new(event_bus.clone());
      installer.lock().unwrap().register_event_handlers().await;

      let app = NodiumApp::new(event_bus);
      debug!("NodiumApp created");

      let options = eframe::NativeOptions::default();
      run_native(Box::new(app), options);
  });
}

