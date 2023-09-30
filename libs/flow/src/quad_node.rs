use crate::{rect::Rect, node::Node};

#[derive(Debug)]
pub struct QuadNode {
    bounds: Rect,
    nodes: Vec<Node>,
    children: Option<[Box<QuadNode>; 4]>,
}

impl QuadNode {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            nodes: Vec::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, node: Node) {
        if self.children.is_none() {
            self.nodes.push(node);
        } else {
            let (x, y) = node.position();
            let mid_x = self.bounds.x() + self.bounds.width() / 2.0;
            let mid_y = self.bounds.y() + self.bounds.height() / 2.0;
            let mut quadrant_index = 0;

            if x > mid_x {
                quadrant_index += 1;
            }
            if y > mid_y {
                quadrant_index += 2;
            }

            if let Some(ref mut children) = &mut self.children {
                children[quadrant_index].insert(node);
            }
        }
    }

    pub fn search(&self, rect: Rect) -> Vec<&Node> {
        let mut result = Vec::new();
        if crate::utils::rect_overlaps(&self.bounds, &rect) {
            for node in &self.nodes {
                if crate::utils::rect_contains(&rect, &node.position()) {
                    result.push(node);
                }
            }
            if let Some(ref children) = &self.children {
                for child in children.iter() {
                    result.extend(child.search(rect));
                }
            }
        }
        result
    }
}
