// nodium_input/src/lib.rs

use nodium_pdk::{Plugin, Node, Service, SideBarElement};

pub struct NodiumInputPlugin;

impl Plugin for NodiumInputPlugin {
    fn name(&self) -> String {
        "nodium_base".to_string()
    }

    // added sidebar elements with html
    fn sidebar_elements(&self) -> Vec<SideBarElement> {
        // icons will have a image and html and  a name on hover and a description
    }

    fn nodes(&self) -> Vec<Node> {
        vec![
            Node {
                name: "debug".to_string(),
                description: "Debug node for displaying messages".to_string(),
                // Other fields
            },
            Node {
                name: "init".to_string(),
                description: "Init node for sending a message when the flow is deployed".to_string(),
                // Other fields
            },
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
