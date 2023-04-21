// nodium_plugin_browser/src/crates_service.rs
use std::sync::{Arc, Mutex};
use nodium_pdk::EventBus;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct CratesList {
    crates: Vec<Crate>,
}

#[derive(Deserialize)]
pub struct Crate {
    name: String,
    version: String,
    // Other properties as needed
}

pub struct CratesService {
    event_bus: Arc<Mutex<EventBus>>,
    crates: Arc<Mutex<Vec<Crate>>>,
}

impl CratesService {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        CratesService {
            event_bus,
            crates: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn fetch_crates(&self) -> Result<(), reqwest::Error> {
        let url = "https://crates.io/api/v1/crates?page=1&per_page=100&sort=downloads";
        let response = get(url)?;
        let crates_list: CratesList = response.json()?;
        let mut crates = self.crates.lock().unwrap();
        *crates = crates_list.crates;
        Ok(())
    }

    pub fn install_crate(&self, krate: &Crate) {
        let event_bus = self.event_bus.lock().unwrap();
        let payload = serde_json::json!({
            "crate_name": krate.name,
            "crate_version": krate.version
        })
        .to_string();
        event_bus.emit("CrateInstall", payload);
    }

    pub fn crates(&self) -> Vec<Crate> {
        self.crates.lock().unwrap().clone()
    }
}
