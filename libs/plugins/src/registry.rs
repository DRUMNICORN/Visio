// libs/plugins/src/plugin_manager.rs

use log::debug;
use nodium_events::NodiumEventBus;
use nodium_pdk::{
    NodiumNode,
    NodiumPlugin,
    // NodiumService,
    NodiumWindow,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct Registry {
    plugins: HashMap<String, Box<dyn NodiumPlugin>>, // nodium_<plugin_name>
                                                     // windows: HashMap<String, Box<dyn NodiumWindow>>, // nodium_<plugin_name>/<window_name>
                                                     // nodes: HashMap<String, NodiumNode>,
                                                     // services: HashMap<String, NodiumService>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            plugins: HashMap::new(),
            // windows: HashMap::new(),
            // nodes: HashMap::new(),
        }
    }

    // TODO: Event Connection from Frontend (Tauri) to Backend (Nodium)
    pub fn register_plugin(&mut self, plugin: Box<dyn NodiumPlugin>) {
        let plugin_name = plugin.name();
        self.plugins.insert(plugin_name.clone().to_owned(), plugin);
    }

    pub fn get_plugin(&self, plugin_name: &str) -> Option<&Box<dyn NodiumPlugin>> {
        self.plugins.get(plugin_name)
    }

    pub fn get_plugins(&self) -> Vec<String> {
        // create a vec and loop over plugins and insert into vec
        let mut plugins = Vec::new();
        let plugins_amount = self.plugins.len();
        debug!("Plugins amount: {}", plugins_amount);
        plugins
    }
}
