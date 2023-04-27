// lib.rs
use std::os::raw::c_char;
use std::ffi::CStr;

// lib.rs
use nodium_sdk::{NodiumApp, TableView, TableRow, TableCell, NodiumPlugin};
use reqwest::Error;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

// CrateInfo and fetch_crates() implementation from previous answer

pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
    type Error = Box<dyn StdError>;

    fn name(&self) -> &'static str {
        "Crates Browser Plugin"
    }

    fn run(&self) -> Result<(), Self::Error> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let crates = fetch_crates().await?;
            let table = create_table_view(crates);

            let app = NodiumApp::new("Crates Browser");
            app.add_view(table);

            app.run();

            Ok(())
        })
    }
}


#[no_mangle]
pub extern "C" fn plugin() -> Box<dyn NodiumPlugin> {
    Box::new(NodiumPluginBrowser)
}

pub fn create_plugin(name: *const c_char) -> Box<dyn NodiumPlugin> {
    let name = unsafe {
        CStr::from_ptr(name)
            .to_string_lossy()
            .into_owned()
    };

    if name == "Crates Browser Plugin" {
        plugin()
    } else {
        panic!("Unknown plugin name");
    }
}
