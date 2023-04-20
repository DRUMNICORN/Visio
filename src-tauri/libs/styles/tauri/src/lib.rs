use nodium_app::{NodiumApp, NodiumRenderer};

// TauriRenderer
#[derive(Clone)]
pub struct TauriRenderer {}

impl TauriRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl NodiumRenderer for TauriRenderer {
    fn run(&self, app: NodiumApp) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
