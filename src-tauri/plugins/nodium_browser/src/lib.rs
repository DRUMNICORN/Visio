// nodium_input/src/lib.rs

use nodium_pdk::{Plugin, Node, Service};

pub struct NodiumBrowser;

impl Plugin for NodiumBrowser {
    fn name(&self) -> String {
        "nodium_browser".to_string()
    }

    fn with_event_bus(&mut self, event_bus: EventBus) {
      self.event_bus = Some(event_bus);
  }

    // will create a browser window
    fn windows(&self) -> Vec<Windows> {
      vec![
          Box::new(CratesWindow::new()),
      ]
  }
    // Implement other methods for additional callbacks
}

// nodium_base/src/lib.rs

#[no_mangle]
pub extern "C" fn plugin() -> Box<dyn nodium_pdk::Plugin> {
    Box::new(NodiumBrowser)
}

pub struct CratesWindow {
  // Window properties and state
}

impl CratesWindow {
  pub fn new() -> Self {
      // Initialize the window properties and state
      CratesWindow {
          // ...
      }
  }

  pub fn render(&self) {
      // Render the window content, e.g., using a WebView
      // ...
  }
}

use reqwest::blocking::get;

fn fetch_crates() -> Result<Vec<Crate>, reqwest::Error> {
    let url = "https://crates.io/api/v1/crates?page=1&per_page=100&sort=downloads";
    let response = get(url)?;
    let crates_list: CratesList = response.json()?;
    Ok(crates_list.crates)
}

use serde::Deserialize;

#[derive(Deserialize)]
struct CratesList {
    crates: Vec<Crate>,
}

#[derive(Deserialize)]
struct Crate {
    name: String,
    version: String,
    // Other properties as needed
}



