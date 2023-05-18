// libs/plugins/src/plugin_manager.rs
use log::debug;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RegistryPlugin {
    pub name: String,
    pub(crate) version: String,
}

pub struct Registry {
    pub plugins: HashMap<String, RegistryPlugin>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: RegistryPlugin) -> Result<(), String> {
        debug!("Registering plugin {} v{}", plugin.name, plugin.version);
        if self.plugins.contains_key(&plugin.name) {
            return Err(format!("Plugin {} already registered", plugin.name));
        }
        self.plugins.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    pub fn get_plugin(&self, plugin_name: &str) -> Option<&String> {
        self.plugins.get(plugin_name)
            .map(|plugin| &plugin.name)
    }

    pub fn get_plugins(&self) -> Vec<&RegistryPlugin> {
        self.plugins.values().collect()
    }
}

