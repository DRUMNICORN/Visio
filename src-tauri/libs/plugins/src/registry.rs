// libs/plugins/src/plugin_manager.rs

use nodium_events::EventBus;
use nodium_pdk::{Plugin, Node, Service, Window};
use std::{collections::HashMap};

pub struct Registry {
    plugins: HashMap<String, Plugin>,
    windows: HashMap<String, Window>,
    nodes: HashMap<String, Node>,
    services: HashMap<String, Service>,
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

  pub fn register_plugin(&mut self, event_bus: EventBus, plugin: Plugin) {
        Plugin::new(event_bus, plugin);
        let plugin_name = plugin.name();
        let nodes = plugin.nodes();
        let services = plugin.services();

        self.plugins.insert(plugin_name.clone(), plugin);

        for node in nodes {
            self.nodes.insert(node.name.clone(), node);
        }

        for service in services {
            self.services.insert(service.name.clone(), service);
        }

        for window in plugin.windows() {
            self.windows.insert(window.name(), window);
        }
    }

    // Add other methods to access and manipulate the registered plugins, nodes, and services
}
