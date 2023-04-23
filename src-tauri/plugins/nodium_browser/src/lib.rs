// nodium_plugin_browser/src/lib.rs
use nodium_pdk::{NodiumPlugin, NodiumNode, NodiumService, NodiumEvent, NodiumWindow, NodiumUiComponent};
use nodium_events::NodiumEventBus;
mod crates_service;
pub use crates_service::CratesService;

use tokio::sync::Mutex;

use log::{debug};
use std::sync::Arc;

pub struct NodiumPluginBrowser {
    event_bus: Option<Arc<Mutex<NodiumEventBus>>>,
}

impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> String {
        "nodium_plugin_browser".to_string()
    }

    fn with_event_bus(&mut self, event_bus: NodiumEventBus) {
        self.event_bus = Some(Arc::new(Mutex::new(event_bus)));
    }

    // will create a browser window
    fn windows(&self, event_bus: Arc<Mutex<NodiumEventBus>>) -> Vec<Box<dyn NodiumWindow>> {
        let event_bus = self.event_bus.as_ref().unwrap().clone();
        let crates_service = Arc::new(CratesService::new(event_bus));
        vec![
            Box::new(CratesBrowserWindow::new(crates_service)),
        ]
    }

    fn nodes(&self, event_bus: Arc<Mutex<NodiumEventBus>>) -> Vec<NodiumNode> {
        vec![]
    }

    fn services(&self, event_bus: Arc<Mutex<NodiumEventBus>>) -> Vec<NodiumService> {
        vec![]
    }
}

pub struct CratesBrowserWindow {
    crates_service: Arc<CratesService>,
    content: NodiumUiComponent,
}

impl CratesBrowserWindow {
    pub fn new(crates_service: Arc<CratesService>) -> Self {
        let content = NodiumUiComponent::Tabs(vec![
            NodiumUiComponent::InputField("Search".to_string(), "search".to_string()),
            NodiumUiComponent::Button("Update".to_string(), "update".to_string()),
        ]);

        CratesBrowserWindow {
            crates_service,
            content,
        }
    }
}

impl NodiumWindow for CratesBrowserWindow {
    fn name(&self) -> String {
        "CratesBrowserWindow".to_string()
    }

    fn icon(&self) -> String {
        "icon.png".to_string()
    }

    fn title(&self) -> String {
        "Crates Browser".to_string()
    }

    fn content(&self) -> NodiumUiComponent {
        self.content.clone()
    }

    fn on_event(&mut self, event_name: &str, _event_data: &str) {
        debug!("CratesBrowserWindow received event: {}", event_name);
        // match event_name {
        //     "update" => {
        //         if let Err(e) = self.crates_service.fetch_crates() {
        //             debug!("Error fetching crates: {}", e);
        //         } else {
        //             let crates = self.crates_service.crates();
        //             // Update the table with the new crates data
        //             // self.content = NodiumUiComponent::Table(crates);
        //         }
        //         }
        //     }
        //     "install" => {
        //         // Install the crate
        //         debug!("Install the crate");
        //     }
        //     _ => {}
        // }
    }
}
