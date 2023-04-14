// nodium_egui/src/lib.rs
use crates_io_api::{SyncClient};
use regex::Regex;
use epi;

use std::sync::{Arc, Mutex};

// constants for extension crates
const EXTENSION_CRATE_PREFIX: &str = "nodium_extension_";

#[derive(Clone)]
pub struct NodiumApp {
    crates: Arc<Mutex<Vec<String>>>,
}

impl NodiumApp {

    // default

    pub fn default() -> Self {
        NodiumApp {
            crates: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn fetch_nodium_extension_crates(crates: Arc<Mutex<Vec<String>>>) {
        println!("Fetching nodium extension crates...");
        let client = SyncClient::new();
        let pattern = Regex::new(&format!("^{}.*", EXTENSION_CRATE_PREFIX)).unwrap();
        // Fetch the list of crates from crates.io

        println!("Fetching crates from crates.io...");
        let query = Some(EXTENSION_CRATE_PREFIX.to_string());
        let result = client.all_crates(query);
        println!("Fetched crates from crates.io");
        // Filter the list of crates to only include nodium extension crates

        let found_crates = match result {
            Ok(crates) => crates,
            Err(e) => {
                println!("Error fetching crates: {}", e);
                return;
            }
        };

        // Update the list of crates
        let mut crates_guard = crates.lock().unwrap();
        for krate in found_crates {
            if pattern.is_match(&krate.name) {
                println!("Found nodium extension crate: {}", krate.name);
                crates_guard.push(krate.name);
            }
        }
    }
}
use tokio::runtime::Handle;

impl epi::App for NodiumApp {
    // ...

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
      // Create a resizable left panel
      egui::SidePanel::left("side_panel")
          .resizable(true)
          .show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
              // Add your sidebar content here
              ui.label("Extensions");

              if ui.button("Fetch crates").clicked() {
                let crates = self.crates.clone();
                let handle = Handle::current();
                handle.spawn(async move {
                    NodiumApp::fetch_nodium_extension_crates(crates).await;
                });
            }
    

              // Display the crates
              ui.separator();
              let crates_guard = self.crates.lock().unwrap();
              for krate in &*crates_guard {
                  ui.label(krate);
              }
            });
          });

      egui::CentralPanel::default().show(ctx, |ui| {
          // Text: "Graph Editor"
          ui.heading("Graph Editor");
      });
  }

    fn name(&self) -> &str {
        "Nodium"
    }
}

