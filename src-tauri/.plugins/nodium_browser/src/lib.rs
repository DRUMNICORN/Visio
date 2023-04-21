// nodium_plugin_browser/src/lib.rs
use nodium_pdk::{Plugin, Node, Service, EventBus, NodiuimWindow, NodiumUiComponent};
use nodium_plugin_browser::crates_service::{CratesService, Crate};
use std::sync::{Arc, Mutex};

pub struct NodiumPluginBrowser {
    event_bus: Option<Arc<Mutex<EventBus>>>,
}

impl Plugin for NodiumPluginBrowser {
    fn name(&self) -> String {
        "nodium_plugin_browser".to_string()
    }

    fn with_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(Arc::new(Mutex::new(event_bus)));
    }

    // will create a browser window
    fn windows(&self) -> Vec<Box<dyn NodiuimWindow>> {
        let event_bus = self.event_bus.as_ref().unwrap().clone();
        let crates_service = Arc::new(CratesService::new(event_bus));
        vec![
            Box::new(CratesBrowserWindow::new(crates_service)),
        ]
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

impl NodiuimWindow for CratesBrowserWindow {
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
        match event_name {
            "update" => {
                if let Err(e) = self.crates_service.fetch_crates() {
                    eprintln!("Error fetching crates: {}", e);
                } else {
                    let crates = self.crates_service.crates();
                    // Update the table with the new crates data
                    // ...
                }
            }
            "install" => {
                // Install the crate
                // ...
            }
            _ => {}
        }
    }
}
