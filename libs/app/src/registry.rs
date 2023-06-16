// libs/plugins/src/plugin_manager.rs

use log::debug;
use nodium_pdk::DynNodiumPlugin;
use std::{collections::HashMap};

pub struct Registry {
    plugins: HashMap<String, DynNodiumPlugin>, // nodium_<plugin_name>
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
    pub fn register_plugin(&mut self, plugin: DynNodiumPlugin) {
        let plugin_name = plugin.name();
        self.plugins.insert(plugin_name.clone().to_owned(), plugin);
    }

    pub fn unregister_plugin(&mut self, plugin_name: &str) {
        self.plugins.remove(plugin_name);
    }

    pub fn unregister_all_plugins(&mut self) {
        self.plugins.clear();
    }

    pub fn get_plugin(&self, plugin_name: &str) -> Option<&DynNodiumPlugin> {
        self.plugins.get(plugin_name)
    }

    pub fn get_plugins(&self) -> Vec<String> {
        // create a vec and loop over plugins and insert into vec
        let mut plugins = Vec::new();
        let plugins_amount = self.plugins.len();
        debug!("Plugins amount: {}", plugins_amount);
        let plugins_names = self.plugins.keys();
        for plugin_name in plugins_names {
            
            plugins.push(plugin_name.to_owned());
        }
        plugins
    }
}