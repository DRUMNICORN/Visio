// libs/plugins/src/plugin_manager.rs

use nodium_pdk::{Plugin, Node, Service};
use std::collections::HashMap;

pub struct Registry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    nodes: HashMap<String, Node>,
    services: HashMap<String, Service>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            plugins: HashMap::new(),
            nodes: HashMap::new(),
            services: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
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
    }

    // Add other methods to access and manipulate the registered plugins, nodes, and services
}
