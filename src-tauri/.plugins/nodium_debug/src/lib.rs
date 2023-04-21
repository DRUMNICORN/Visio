// nodium_debug/src/lib.rs

use nodium_pdk::{Plugin, Node, Service};

pub struct NodiumDebugPlugin;

impl Plugin for NodiumDebugPlugin {
    fn name(&self) -> String {
        "nodium_base".to_string()
    }

    fn nodes(&self) -> Vec<Node> {
        vec![
            Node {
                name: "debug".to_string(),
                description: "Debug node for displaying messages".to_string(),
                // Other fields
            }
        ]
    }

    fn services(&self) -> Vec<Service> {
        vec![]
    }

    // Implement other methods for additional callbacks
}

// nodium_base/src/lib.rs

#[no_mangle]
pub extern "C" fn plugin() -> Box<dyn nodium_pdk::Plugin> {
    Box::new(NodiumInputPlugin)
}
