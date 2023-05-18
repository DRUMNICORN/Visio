// plugins/nodium_browser/src/lib.rs
use nodium_pdk::{NodiumPlugin, PluginInterface};
use std::boxed::Box;
use abi_stable::{DynTrait, std_types::RString};

pub struct HelloWorld;

impl NodiumPlugin for HelloWorld {
    fn name(&self) -> RString {
        "Hello World".into()
    }

    fn version(&self) -> RString {
        "0.1.0".into()
    }
}

impl HelloWorld {
    pub fn new() -> Box<dyn NodiumPlugin> {
        Box::new(HelloWorld)
    }
}

#[no_mangle]
pub static PLUGIN_INTERFACE: PluginInterface = PluginInterface {
    constructor: constructor,
    drop: drop_plugin,
};

extern "C" fn constructor() -> Box<dyn NodiumPlugin> {
    HelloWorld::new()
}

extern "C" fn drop_plugin(plugin: Box<dyn NodiumPlugin>) {
    // Plugin is dropped automatically
}
