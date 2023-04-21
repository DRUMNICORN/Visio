// libs/plugins/src/plugin_manager.rs

use nodium_events::NodiumEvents;
use nodium_pdk::{NodiuimPlugin, NodiuimNode, NodiuimService, NodiuimWindow};
use tokio::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

pub struct Registry {
    plugins: HashMap<String, Box<dyn NodiuimPlugin>>,
    windows: HashMap<String, NodiuimWindow>,
    nodes: HashMap<String, NodiuimNode>,
    services: HashMap<String, NodiuimService>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            plugins: HashMap::new(),
            nodes: HashMap::new(),
            services: HashMap::new(),
            windows: HashMap::new(),
        }
    }

  pub fn register_plugin(&mut self, event_bus: Arc<Mutex<NodiumEvents>>, plugin: Box<dyn NodiuimPlugin>) {
        
        let plugin_name = plugin.name();
        let nodes = plugin.nodes(event_bus.clone());
        let services = plugin.services(event_bus.clone());
        let windows = plugin.windows(event_bus);

        self.plugins.insert(plugin_name.clone(), plugin);

        for node in nodes {
            self.nodes.insert(node.name.clone(), node);
        }

        for service in services {
            self.services.insert(service.name.clone(), service);
        }

        for window in windows {
            self.windows.insert(window.name.clone(), window);
        }
    }

    // Add other methods to access and manipulate the registered plugins, nodes, and services
}
