use crate::{output::Output, input::Input};


#[derive(Debug)]
pub struct Node {
    position: (f32, f32),
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

impl Node {
    pub fn new(position: (f32, f32)) -> Self {
        Self {
            position,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }
}

