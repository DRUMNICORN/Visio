use eframe::run_native;
use nodium_app::NodiumApp;
use nodium_events::EventBus;
use tokio::runtime::Runtime;

use nodium_core::crate_installer::CrateInstaller;

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
      let installer = CrateInstaller::new(event_bus.clone());
      installer.lock().unwrap().register_event_handlers().await;

      // Emit the "CrateInstall" event with a payload
      // event_bus.emit("CrateInstall", String::from("Payload for CrateInstall")).await;

      // Main application logic
      let app = NodiumApp::new(event_bus);
      debug!("NodiumApp created");

      let options = eframe::NativeOptions::default();
      run_native(Box::new(app), options);
  });
}

