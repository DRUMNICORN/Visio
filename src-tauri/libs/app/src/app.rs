use log::debug;
use nodium_events::NodiumEventBus;
use nodium_pdk::NodiumEvent;
use nodium_plugins::NodiumPlugins;
use tokio::sync::Mutex;

use crate::NodiumView;
use std::sync::Arc;

pub struct NodiumApp {
    event_bus: Arc<Mutex<NodiumEventBus>>,
    view: Box<dyn NodiumView>,
    plugin_manager: Arc<Mutex<NodiumPlugins>>,
}

impl NodiumApp {
    pub async fn init(
        event_bus: Arc<Mutex<NodiumEventBus>>,
        view: Box<dyn NodiumView>,
    ) -> Self {
        debug!("App init");

        let plugin_manager = NodiumPlugins::new(event_bus.clone()).await;

        NodiumApp {
            event_bus,
            view,
            plugin_manager,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.view.run()?;
        Ok(())
    }

    pub async fn event(&self, name: String, payload: String) {
        debug!("Event: {} - {}", name, payload);
        // self.event_bus.lock().await.send(&name, payload.to_string());
        self.event_bus
            .lock()
            .await
            .emit(&name, payload.to_string())
            .await;
    }
}
