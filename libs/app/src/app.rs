use tokio::sync::Mutex;
use std::sync::Arc;

use crate::plugins::NodiumPlugins;
use crate::flows::NodiumFlows;

pub struct NodiumApp {
    pub flows: Arc<Mutex<NodiumFlows>>,
    pub plugins: Arc<Mutex<NodiumPlugins>>,
}

impl NodiumApp {
    pub fn new() -> Arc<Mutex<Self>> {
        let plugins = NodiumPlugins::new();
        let flows = NodiumFlows::new();

        let app = NodiumApp {
            plugins,
            flows,
        };

        Arc::new(Mutex::new(app))
    }
}
