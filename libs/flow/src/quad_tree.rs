use crate::{rect::Rect, node::Node, quad_node::QuadNode};


#[derive(Debug)]
pub struct QuadTree {
    root: Option<Box<QuadNode>>,
}

impl QuadTree {
    pub fn new(bounds: Rect) -> Self {
        Self {
            root: Some(Box::new(QuadNode::new(bounds))),
        }
    }

    pub fn insert(&mut self, node: Node) {
        if let Some(root) = &mut self.root {
            root.insert(node);
        }
    }

    pub fn search(&self, rect: Rect) -> Vec<&Node> {
        if let Some(root) = &self.root {
            return root.search(rect);
        }
        Vec::new()
    }

    
}

