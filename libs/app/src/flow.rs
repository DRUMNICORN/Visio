
use nodium_pdk::node::NodiumNode;
use std::collections::HashMap;
use log::debug;

pub struct NodiumFlow {
    pub nodes: Vec<NodiumNode>,
}

impl NodiumFlow {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: NodiumNode) {
        self.nodes.push(node);
    }

    pub fn list_connected_types(&self) {
        let mut connected_types: HashMap<String, String> = HashMap::new();

        for node in &self.nodes {
            for (input_key, input_value) in &node.input_params {
                connected_types.insert(input_key.clone(), input_value.clone());
            }

            for (output_key, output_value) in &node.output_params {
                connected_types.insert(output_key.clone(), output_value.clone());
            }
        }

        debug!("Connected Types:");
        for (key, value) in &connected_types {
            debug!("{}: {}", key, value);
        }
    }
}