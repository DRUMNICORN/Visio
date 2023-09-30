use std::sync::{Arc, Mutex};
use nodium_flow::flow::Flow;

pub struct NodeRenderer {
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    // Add more node rendering-related fields as needed.
}

impl NodeRenderer {
    pub fn new(device: Arc<Mutex<wgpu::Device>>, queue: Arc<Mutex<wgpu::Queue>>) -> Self {
        Self {
            device,
            queue,
            // Initialize other node rendering components here.
        }
    }

    pub fn render(&self, flow: Arc<Mutex<Flow>>) {

        let nodes = flow.lock();
        match nodes {
            Ok(nodes) => {
                let nodes = nodes.nodes(true);
                for node in nodes {
                    self.render_node(node);
                }
            }
            Err(_) => {
                // Handle the error here.
            }
        }
    }

    pub fn render_node(&self, node: &nodium_flow::node::Node) {
        // Render the node here.
        let device = self.device.lock().unwrap();
        let queue = self.queue.lock().unwrap();

    }
}
