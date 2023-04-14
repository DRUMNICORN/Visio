// nodium_egui/src/style_registry.rs

use crate::node_style::NodeStyle;
use std::collections::HashMap;

pub struct StyleRegistry {
    styles: HashMap<String, NodeStyle>,
}

impl StyleRegistry {
    pub fn new() -> Self {
        StyleRegistry {
            styles: HashMap::new(),
        }
    }

    pub fn register_style(&mut self, node_type: &str, style: NodeStyle) {
        self.styles.insert(node_type.to_string(), style);
    }

    pub fn get_style(&self, node_type: &str) -> Option<&NodeStyle> {
        self.styles.get(node_type)
    }
}
