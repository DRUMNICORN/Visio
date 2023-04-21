use nodium_events::NodiumEventBus;
use nodium_plugins::NodiumPlugins;
use tokio::sync::Mutex;

use crate::NodiumView;
use std::sync::Arc;

pub struct NodiumApp {
    plugin_manager: Arc<Mutex<NodiumPlugins>>,
    event_manager: Arc<Mutex<NodiumEventBus>>,
    view_manager: Box<dyn NodiumView>,
}

impl Clone for NodiumApp {
    fn clone(&self) -> Self {
        NodiumApp {
            plugin_manager: self.plugin_manager.clone(),
            event_manager: self.event_manager.clone(),
            view_manager: self.view_manager.clone_box(),
        }
    }
}

impl NodiumApp {
    pub async fn new(renderer: Box<dyn NodiumView>) -> Self {
        let event_notifier = |event_name: &str, payload: &str| {
            println!("{}: {}", event_name, payload);
        };
        let event_bus = NodiumEventBus::new(Box::new(event_notifier));
        let event_bus_clone = event_bus.clone();
        NodiumApp {
            plugin_manager: NodiumPlugins::new(event_bus).await,
            event_manager: event_bus_clone,
            view_manager: renderer,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.view_manager.run(Arc::new(Mutex::new(self.clone())))
    }

    pub async fn event(&self, name: String, payload: String) {
        self.event_manager
            .lock()
            .await
            .send(&name, payload.to_string());
    }
}
