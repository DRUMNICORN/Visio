// nodium_plugin_browser/src/crates_service.rs
use nodium_events::NodiumEventBus;
use reqwest::blocking::get;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct CratesList {
    crates: Vec<Crate>,
}

#[derive(Deserialize, Clone)]
pub struct Crate {
    name: String,
    version: String,
    // Other properties as needed
}

pub struct CratesService {
    event_bus: Arc<Mutex<NodiumEventBus>>,
    crates: Arc<Mutex<Vec<Crate>>>,
}

impl CratesService {
    pub fn new(event_bus: Arc<Mutex<NodiumEventBus>>) -> Self {
        CratesService {
            event_bus,
            crates: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn fetch_crates(&self) -> Result<(), reqwest::Error> {
        let url = "https://crates.io/api/v1/crates?page=1&per_page=100&sort=downloads";
        let response = get(url)?;
        let crates_list: CratesList = response.json()?;
        let mut crates = self.crates.lock().await;
        *crates = crates_list.crates;
        Ok(())
    }

    pub async fn install_crate(&self, krate: &Crate) {
        let payload = serde_json::json!({
            "crate_name": krate.name,
            "crate_version": krate.version
        })
        .to_string();
        let event_bus = self.event_bus.clone();
        tokio::spawn(async move {
            event_bus.lock().await.emit("install_crate", payload).await;
        });
    }

    pub async fn crates(&self) -> Vec<Crate> {
        self.crates.lock().await.clone()
    }
}
