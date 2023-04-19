use nodium_app::NodiumApp;
use nodium_tauri::TauriRenderer;
use nodium_events::EventBus;
use nodium_plugins::Plugins;
use tokio::runtime::Runtime;
use env_logger::Builder;
use log::{debug, info, LevelFilter};

use std::sync::Arc;
use tauri::{Manager, AppHandle};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

      // image all code here
      
          // tauri::Builder::default()
  
          // .invoke_handler(tauri::generate_handler![])
          //     .manage(event_bus) // Add the event_bus to Tauri state
          //     .run(tauri::generate_context!())
          //     .expect("error while running tauri application");

        let renderer = TauriRenderer::new();
        let mut nodium_app = NodiumApp::new(event_bus.clone(), Box::new(renderer));
        debug!("NodiumApp created");

        // we wannt to connect the event bus to the tauri app
        // so we can send events from the app to the event bus
        // and from the event bus to the app
        // we can do this by creating a new event bus
        // and then sending the event bus to the app
        // and then sending the event bus to the tauri app

        // we start by creating a new event bus for the tauri app
        let tauri_event_bus = EventBus::new();

        // we then send the event bus to the app
        nodium_app.set_event_bus(tauri_event_bus.clone());

        // we then send the event bus to the tauri app
        tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            // we then send the event bus to the tauri app
            handle.listen_global("request", move |event| {
                // we then send the event bus to the tauri app
                // unwarp event name and  
            });

            Ok(())
        })           .invoke_handler(tauri::generate_handler![])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
        

    });

}
