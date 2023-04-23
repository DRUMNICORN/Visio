// nodium_plugin_browser/src/lib.rs
use nodium_pdk::{NodiumPlugin, NodiumNode, NodiumService, NodiumEvent, NodiumWindow, NodiumUiComponent};
use nodium_events::NodiumEventBus;
mod crates_service;
pub use crates_service::CratesService;

use tokio::sync::Mutex;

use log::{debug};
use std::sync::Arc;

pub struct NodiumPluginBrowser {
    // events: Option<Arc<Mutex<NodiumEventBus>>>,
}


pub struct CratesBrowserWindow {
    crates_service: Arc<CratesService>,
    content: NodiumUiComponent,
}
