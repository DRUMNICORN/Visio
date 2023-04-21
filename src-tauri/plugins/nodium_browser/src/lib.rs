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



// async fn _fetch_nodium_extension_crates(
//   crates: Arc<Mutex<HashSet<WrappedCrate>>>,
//   fetching: Arc<Mutex<bool>>,
// ) {
//   // Create a crates.io client
//   let client = SyncClient::new();
//   let pattern = Regex::new(&format!("^{}.*", PLUGIN_CRATE_PREFIX)).unwrap();

//   // Fetch the list of crates from crates.io
//   let query = Some(PLUGIN_CRATE_PREFIX.to_string());
//   let result = client.all_crates(query);

//   // Filter the list of crates to only include nodium extension crates
//   let found_crates = match result {
//       Ok(crates) => crates,
//       Err(e) => {
//           debug!("Error fetching crates: {}", e);
//           let mut fetching = fetching.lock().unwrap();
//           *fetching = false;
//           return;
//       }
//   };

//   // Update the list of crates
//   let mut crates_guard = crates.lock().unwrap();
//   for krate in found_crates {
//       if pattern.is_match(&krate.name) {
//           debug!("Found nodium extension crate: {}", krate.name);
//           crates_guard.insert(WrappedCrate(krate));
//       }
//   }

//   let mut fetching = fetching.lock().unwrap();
//   *fetching = false;
// }

// fn install_crate(&self, krate: WrappedCrate) {
//   let krate = krate.0;
//   let event_bus = self.event_bus.clone();
//   let payload = serde_json::json!({
//     "crate_name": krate.clone().name,
//     "crate_version": krate.clone().max_version
//   })
//   .to_string();
//   debug!("Installing crate: {}", payload);
//   // warp in tokio task
//   tokio::spawn(async move {
//       event_bus.emit("CrateInstall", payload).await;
//   });
// }


const PLUGIN_CRATE_PREFIX: &str = "nodium_";


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]