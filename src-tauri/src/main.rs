use env_logger::Builder;
use log::{debug, error, LevelFilter};
use nodium_app::NodiumApp;
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
                let nodium_app = NodiumApp::new(Box::new(NodiumViewTauri::new(handle)));
                match nodium_app.await.run() {
                    Ok(_) => {
                        debug!("NodiumApp exited successfully");
                    }
                    Err(e) => {
                        error!("NodiumApp exited with error: {}", e);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
