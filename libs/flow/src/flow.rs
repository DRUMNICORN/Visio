use crate::{rect::Rect, node::Node, quad_tree::QuadTree};

#[derive(Debug)]
pub struct Flow {
    position: (f32, f32),
    zoom: f32,
    quadtree: QuadTree,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            zoom: 1.0,
            quadtree: QuadTree::new(Rect::new(0.0, 0.0, 1000.0, 1000.0)), // Adjust the bounds as needed.
        }
    }

    pub fn insert_node(&mut self, node: Node) {
        self.quadtree.insert(node);
    }

    pub fn search_nodes(&self, rect: Rect) -> Vec<&Node> {
        self.quadtree.search(rect)
    }

    pub fn nodes(&self, in_vew: bool) -> Vec<&Node> {
        let mut result = Vec::new();
        if in_vew {
            let rect = self.view_rect();
            result.extend(self.search_nodes(rect));
        } else {
            result.extend(self.search_nodes(Rect::new(0.0, 0.0, 1000.0, 1000.0)));
        }
        result
    }

    pub fn view_rect(&self) -> Rect {
        let (x, y) = self.position;
        let width = 1000.0 / self.zoom;
        let height = 1000.0 / self.zoom;
        Rect::new(x, y, width, height)
    }

}

