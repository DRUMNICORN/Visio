use nodium_events::EventBus;

use crate::{warp::WrappedCrate, NodiumRenderer};
use crates_io_api::SyncClient;
use log::debug;
use regex::Regex;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

const PLUGIN_CRATE_PREFIX: &str = "nodium_";

pub struct NodiumApp {
    search_query: String,
    crates: Arc<Mutex<HashSet<WrappedCrate>>>,
    event_bus: Arc<EventBus>,
    renderer: Box<dyn NodiumRenderer>,
    fetching: Arc<Mutex<bool>>,
}

impl Clone for NodiumApp {
    fn clone(&self) -> Self {
        NodiumApp {
            event_bus: self.event_bus.clone(),
            renderer: self.renderer.clone_box(),
            crates: self.crates.clone(),
            search_query: self.search_query.clone(),
            fetching: self.fetching.clone(),
        }
    }
}

impl NodiumApp {
    pub fn new(event_bus: Arc<EventBus>, renderer: Box<dyn NodiumRenderer>) -> Self {
        let crates = Arc::new(Mutex::new(HashSet::new()));
        let fetching = Arc::new(Mutex::new(true));
        let event_bus = event_bus.clone();
        let crates_clone = crates.clone();
        let fetching_clone = fetching.clone();
        // tokio::spawn(async move {
        //     Self::fetch_nodium_extension_crates(crates_clone, fetching_clone).await;
        // });
        NodiumApp {
            search_query: String::new(),
            crates,
            event_bus,
            renderer: renderer,
            fetching,
        }
    }

    pub fn run(&self) {
        self.renderer.run(self.clone());
    }

    async fn _fetch_nodium_extension_crates(
        crates: Arc<Mutex<HashSet<WrappedCrate>>>,
        fetching: Arc<Mutex<bool>>,
    ) {
        // Create a crates.io client
        let client = SyncClient::new();
        let pattern = Regex::new(&format!("^{}.*", PLUGIN_CRATE_PREFIX)).unwrap();

        // Fetch the list of crates from crates.io
        let query = Some(PLUGIN_CRATE_PREFIX.to_string());
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

    fn install_crate(&self, krate: WrappedCrate) {
        let krate = krate.0;
        let event_bus = self.event_bus.clone();
        let payload = serde_json::json!({
          "crate_name": krate.clone().name,
          "crate_version": krate.clone().max_version
        })
        .to_string();
        debug!("Installing crate: {}", payload);
        // warp in tokio task
        tokio::spawn(async move {
            event_bus.emit("CrateInstall", payload).await;
        });
    }
}
