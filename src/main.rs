use env_logger::Builder;
use log::{error, debug};
use log::LevelFilter;
use nodium_app::NodiumApp;

#[cfg(feature = "egui")]
use nodium_egui::NodiumViewEgui;

#[cfg(feature = "tauri")]
use nodium_tauri::NodiumViewTauri;

#[cfg(feature = "nodium-console")]
use nodium_console::NodiumConsole;


#[tokio::main]
async fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug) // Change this to the desired log level
        .init();
    debug!("Welcome to Nodium!\n");

    debug!("App started");
    #[cfg(feature = "egui")]
    {
        let view = NodiumViewEgui::new();
        let app = NodiumApp::new(Box::new(view));
    }

    #[cfg(feature = "tauri")]
    {
        let view = NodiumViewTauri::new();
        let app = NodiumApp::new(Box::new(view));
    }

    #[cfg(not(any(feature = "egui", feature = "tauri")))]
    {
      let app = NodiumApp::new();
      let view = NodiumConsole::new(app);
      
      match view.run().await {
            Ok(_) => {
                debug!("App exited successfully");
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
    // Ok(())
    ()
}



// Dummy view

struct NodiumViewDummy;

use nodium_app::NodiumView;

#[async_trait::async_trait]
impl NodiumView for NodiumViewDummy {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
      Ok(())
    }

  }