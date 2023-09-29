
#[derive(Debug)]
pub struct Flow {
    position: (f32, f32),
    zoom: f32,
    nodes: Vec<Node>,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            zoom: 1.0,
            nodes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    name: String,
    value: f32,
    node: Node,
}

#[derive(Debug)]
pub struct Output {
    name: String,
    node: Node,
}

#[derive(Debug)]
pub struct Node {
    position: (f32, f32),
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}
