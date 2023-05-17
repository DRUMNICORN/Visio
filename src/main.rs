use env_logger::Builder;
use log::error;
use log::LevelFilter;
use nodium_app::NodiumApp;

#[cfg(feature = "egui")]
use nodium_egui::NodiumViewEgui;

use nodium_pdk::NodiumLayout;
#[cfg(feature = "tauri")]
use nodium_tauri::NodiumViewTauri;

#[cfg(feature = "nodium-console")]
use nodium_console::NodiumViewConsole;


#[tokio::main]
async fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug) // Change this to the desired log level
        .init();
    println!("Welcome to Nodium!\n");

    println!("App started");
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
      let view = NodiumViewConsole::new(app);
      
      match view.run().await {
            Ok(_) => {
                println!("App exited successfully");
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
use nodium_pdk::NodiumWindow;

#[async_trait::async_trait]
impl NodiumView for NodiumViewDummy {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
      Ok(())
    }

    fn add_window(&self,_window:Box<dyn NodiumWindow>) -> Result<(),Box<dyn std::error::Error> >  {
      Ok(())

    }

    fn remove_window(&self,_window:Box<dyn NodiumWindow> ,) -> Result<(),Box<dyn std::error::Error> >  {
      Ok(())

    }

    fn update_window(&self,_window:Box<dyn NodiumWindow> ,) -> Result<(),Box<dyn std::error::Error> >  {
      Ok(())

    }

    fn set_layout(&self,_layout:NodiumLayout) -> Result<(),Box<dyn std::error::Error> >  {
      Ok(())

    }

    fn focus_window(&self, _window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

  }