// lib.rs

use  nodium_pdk::NodiumPlugin;

pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> &'static str {
        "Crates Browser Plugin"
    }

}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn NodiumPlugin {
    Box::into_raw(Box::new(NodiumPluginBrowser))
}
