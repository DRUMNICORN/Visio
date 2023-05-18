// path: plugins/nodium_browser/src/lib.rs

use abi_stable::StableAbi;
use nodium_pdk::{NodiumPlugin, NodiumPluginWrapper};

#[repr(C)]
#[derive(StableAbi)]
pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> String {
        "Browser".to_string()
    }

    fn version(&self) -> String {
        "0.1.0".to_string()
    }
}

impl NodiumPluginBrowser {
    pub fn new() -> Self {
        NodiumPluginBrowser
    }
}

#[no_mangle]
pub extern "C" fn plugin_ctor() -> *mut NodiumPluginWrapper {
    let plugin = NodiumPluginBrowser::new();
    let wrapper = NodiumPluginWrapper::new(Box::new(plugin));
    Box::into_raw(Box::new(wrapper))
}
