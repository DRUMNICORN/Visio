use crate::node::Node;

#[derive(Debug)]
pub struct Output {
    name: String,
    node: Node,
}

impl Output {
    pub fn new(name: String, node: Node) -> Self {
        Self { name, node }
    }
}
