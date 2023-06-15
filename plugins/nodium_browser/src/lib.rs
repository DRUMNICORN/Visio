// lib.rs
mod extract_crate_file;
pub use extract_crate_file::extract_crate_file;
use  nodium_pdk::{NodiumPlugin, DynNodiumPlugin};

pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
    extern fn name(&self) -> nodium_pdk::StaticStr {
        nodium_pdk::StaticStr::new("browser")
    }

}

#[no_mangle]
pub extern "C" fn create_plugin() -> DynNodiumPlugin {
   DynNodiumPlugin::new(&NodiumPluginBrowser)
}
