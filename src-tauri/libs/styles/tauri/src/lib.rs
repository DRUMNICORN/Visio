use nodium_app::{NodiumApp, NodiumRenderer};

// TauriRenderer
#[derive(Clone)]
pub struct TauriRenderer {
    _app: NodiumAppTauri
}

impl TauriRenderer {
    pub fn new() -> Self {
        Self {
            _app: NodiumAppTauri::new()
        }
    }
}

impl NodiumRenderer for TauriRenderer {
  fn run(&self, app: nodium_app::NodiumApp) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}


// NodiumAppTauri
#[derive(Clone)]
struct NodiumAppTauri {
  // new 
    // ...
}

impl NodiumAppTauri {
    pub fn new() -> Self {
        Self {
            // new
        }
    }
}