// nodium_egui/src/node.rs

use crate::style_registry::StyleRegistry;

pub struct Node {
    // ...
    node_type: String,
}

impl Node {
    pub fn new(node_type: &str) -> Self {
        // ...
        Node {
            // ...
            node_type: node_type.to_string(),
        }
    }

    pub fn draw(&self, style_registry: &StyleRegistry) {
        if let Some(style) = style_registry.get_style(&self.node_type) {
            // Apply the style properties to the Egui components
            // For example, set the background color, icon, and label
        } else {
            // Fallback to default style
        }
    }
}
