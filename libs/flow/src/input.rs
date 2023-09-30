use crate::node::Node;


#[derive(Debug)]
pub struct Input {
    name: String,
    value: f32,
    node: Node,
}

impl Input {
    pub fn new(name: String, value: f32, node: Node) -> Self {
        Self { name, value, node }
    }
}