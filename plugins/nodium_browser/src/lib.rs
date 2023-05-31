// lib.rs
mod extract_crate_file;
pub use extract_crate_file::extract_crate_file;
use  nodium_pdk::NodiumPlugin;

pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> &'static str {
        "browser"
    }

}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn NodiumPlugin {
    Box::into_raw(Box::new(NodiumPluginBrowser))
}
