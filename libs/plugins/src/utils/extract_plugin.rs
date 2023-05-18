// libs/plugins/src/utils/extract_plugin.rs
use nodium_pdk::{NodiumPlugin, PluginInterface};
use std::path::Path;
use libloading::{Library, Symbol};
use log::debug;
use abi_stable::{DynTrait, std_types::RBoxError};

pub unsafe fn extract_plugin(lib_path: &Path) -> Result<DynTrait<dyn NodiumPlugin, PluginInterface>, RBoxError> {
    let lib_path = lib_path.to_str().unwrap();
    let lib = Library::new(lib_path)?;
    unsafe {
        debug!("Loading plugin from: {}", lib_path.to_string());
        let plugin_ctor: Symbol<extern "C" fn() -> Box<dyn NodiumPlugin>> = lib.get(b"plugin_ctor")?;
        debug!("Plugin loaded successfully");
        let plugin_trait_object = plugin_ctor();
        debug!("Plugin constructor called successfully");
        Ok(DynTrait::from_value(plugin_trait_object))
    }
}
