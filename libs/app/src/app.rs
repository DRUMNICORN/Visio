use nodium_plugins::NodiumPlugins;
use tokio::sync::Mutex;

use std::sync::Arc;

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
