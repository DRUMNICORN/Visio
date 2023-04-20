use env_logger::Builder;
use log::{debug, info, LevelFilter};
use nodium_app::NodiumApp;
use nodium_events::EventBus;
use nodium_plugins::Plugins;
use nodium_tauri::TauriRenderer;
use tokio::runtime::Runtime;

use std::sync::Arc;
use tauri::{AppHandle, Manager};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug) // Change this to the desired log level
        .init();
    println!("Welcome to Nodium!\n");
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        // event notifier

        
        
        
        
        tauri::Builder::default()
        .setup(|app| {
            let event_notifier = |event_name: &str, payload: &str| {
              println!("{}: {}", event_name, payload);
            };
            let event_bus = EventBus::new(Box::new(event_notifier));
            let plugins = Plugins::new(event_bus.clone()).await;
            plugins.lock().await.listen().await;
                plugins.lock().await.reload().await;
              // we get the event bus from the app
              let renderer = TauriRenderer::new();
              let mut nodium_app = NodiumApp::new(event_bus.clone(), Box::new(renderer));
              //
              let handle = app.handle();
                // we then send the event bus to the app

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
