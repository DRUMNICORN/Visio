// libs/nodium_ui/src/lib.rs TODO: https://docs.github.com/en/get-started/using-git/splitting-a-subfolder-out-into-a-new-repository
use crates_io_api::{Crate, SyncClient};
use egui::Ui;
use log::debug;
use regex::Regex;

use nodium_events::EventBus;
use serde_json::json;

use std::hash::{Hash, Hasher};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

// constants for extension crates
const EXTENSION_CRATE_PREFIX: &str = "nodium"; // prefix for extension crates on crates.io are "nodium_"

pub struct WrappedCrate(pub Crate);

impl PartialEq for WrappedCrate {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Eq for WrappedCrate {}

impl Hash for WrappedCrate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl Clone for WrappedCrate {
    fn clone(&self) -> Self {
        WrappedCrate(self.0.clone())
    }
}

#[derive(Clone)]
pub struct NodiumApp {
    search_query: String,
    crates: Arc<Mutex<HashSet<WrappedCrate>>>,
    event_bus: Arc<EventBus>,
    fetching: Arc<Mutex<bool>>,
}

impl NodiumApp {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        let crates = Arc::new(Mutex::new(HashSet::new()));
        let fetching = Arc::new(Mutex::new(true));
        let event_bus = event_bus.clone();
        let crates_clone = crates.clone();
        let fetching_clone = fetching.clone();
        tokio::spawn(async move {
            Self::fetch_nodium_extension_crates(crates_clone, fetching_clone).await;
        });
        NodiumApp {
            search_query: String::new(),
            crates,
            event_bus,
            fetching,
        }
    }

    // Inside NodiumApp implementation
    pub fn run(&self) {
        let _app = eframe::run_native(Box::new(self.clone()), eframe::NativeOptions::default());
    }

    async fn fetch_nodium_extension_crates(
        crates: Arc<Mutex<HashSet<WrappedCrate>>>,
        fetching: Arc<Mutex<bool>>,
    ) {
        // Create a crates.io client
        let client = SyncClient::new();
        let pattern = Regex::new(&format!("^{}.*", EXTENSION_CRATE_PREFIX)).unwrap();

        // Fetch the list of crates from crates.io
        let query = Some(EXTENSION_CRATE_PREFIX.to_string());
        let result = client.all_crates(query);

        // Filter the list of crates to only include nodium extension crates
        let found_crates = match result {
            Ok(crates) => crates,
            Err(e) => {
                debug!("Error fetching crates: {}", e);
                let mut fetching = fetching.lock().unwrap();
                *fetching = false;
                return;
            }
        };

        // Update the list of crates
        let mut crates_guard = crates.lock().unwrap();
        for krate in found_crates {
            if pattern.is_match(&krate.name) {
                debug!("Found nodium extension crate: {}", krate.name);
                crates_guard.insert(WrappedCrate(krate));
            }
        }

        let mut fetching = fetching.lock().unwrap();
        *fetching = false;
    }

    fn install_button(&self, krate: Crate) -> impl FnOnce(&mut Ui) {
        let event_bus = self.event_bus.clone();
        move |ui: &mut Ui| {
            ui.label(&krate.name);
            if ui.button("Install").clicked() {
                let payload = json!({ "crate_name": krate.clone().name }).to_string();
                debug!("Installing crate: {}", payload);
                // warp in tokio task
                tokio::spawn(async move {
                    event_bus.emit("CrateInstall", payload).await;
                });
            }
        }
    }
}
use tokio::runtime::Handle;

impl epi::App for NodiumApp {
    fn name(&self) -> &str {
        "Nodium"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        // Create a resizable left panel
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    // Add your sidebar content here
                    ui.label("Extensions");
                    if ui.button("Fetch extension crates").clicked() {
                        let crates = self.crates.clone();
                        let fetching = Arc::new(Mutex::new(true));
                        self.fetching = fetching.clone();
                        let handle = Handle::current();
                        handle.spawn(async move {
                            NodiumApp::fetch_nodium_extension_crates(crates, fetching).await;
                        });
                    }

                    // Display the crates with install buttons
                    ui.separator();
                    let crates_guard = self.crates.lock().unwrap();

                    // Search pannel on top of the crates
                    ui.horizontal(|ui| {
                        ui.label("Search:");
                        ui.text_edit_singleline(&mut self.search_query);
                        if let Ok(fetching) = self.fetching.lock() {
                            if *fetching {
                                ui.label("Fetching...");
                            } else {
                                ui.label("Done!");
                            }
                        }
                    });

                    // Filter crates based on search query
                    let filtered_crates = crates_guard
                        .iter()
                        .filter(|krate| krate.0.name.contains(&self.search_query))
                        .collect::<Vec<_>>();
                    for krate in filtered_crates {
                        self.install_button(krate.0.clone())(ui);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Text: "Graph Editor"
            ui.heading("Graph Editor");
            // Text: "Welcome to Nodium!"
            ui.label("Welcome to Nodium!");
        });
    }
}
