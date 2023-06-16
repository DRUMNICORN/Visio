use tokio::sync::Mutex;
use std::sync::Arc;

use crate::plugins::NodiumPlugins;

pub struct NodiumApp {
    // event_bus: Arc<Mutex<NodiumEventBus>>,
    pub plugins: Arc<Mutex<NodiumPlugins>>,
}

impl NodiumApp {
    pub fn new() -> Arc<Mutex<Self>> {
        let plugins = NodiumPlugins::new();
        // default view is console

        let app =  NodiumApp {
            plugins,
        };

        Arc::new(Mutex::new(app))
    }
}
