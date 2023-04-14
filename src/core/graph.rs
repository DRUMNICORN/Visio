use std::collections::HashMap;
use crate::core::node::Node;

pub struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.get_id().to_owned(), node);
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn remove_node(&mut self, id: &str) {
        self.nodes.remove(id);
    }
}
