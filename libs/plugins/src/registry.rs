// libs/plugins/src/plugin_manager.rs
use log::debug;
use std::collections::HashMap;
use nodium_pdk::NodiumPluginObject;

pub struct Registry {
    plugins: HashMap<String, Box<dyn NodiumPluginObject>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn NodiumPluginObject>) -> Option<&Box<dyn NodiumPluginObject>> {
        let plugin_name = plugin.name();
        self.plugins.insert(plugin_name.to_string(), plugin);
        self.get_plugin(&plugin_name)
    }

    pub fn get_plugin(&self, plugin_name: &str) -> Option<&Box<dyn NodiumPluginObject>> {
        self.plugins.get(plugin_name)
    }

    pub fn get_plugins(&self) -> Vec<String> {
        let mut plugins = Vec::new();
        for plugin_name in self.plugins.keys() {
            plugins.push(plugin_name.clone());
        }
        let plugins_amount = plugins.len();
        debug!("Plugins amount: {}", plugins_amount);
        plugins
    }
}

