use libloading::{Library, Symbol};
use log::debug;
use nodium_pdk::NodiumPluginObject;

pub fn extract_plugin(lib: Library) -> Result<Box<dyn NodiumPluginObject>, Box<dyn std::error::Error>> {
    debug!("Extracting plugin");
    let plugin: Symbol<unsafe extern "C" fn() -> *mut dyn NodiumPluginObject> =
        unsafe { lib.get(b"plugin")? };
    let plugin_ptr = unsafe { plugin() };
    if plugin_ptr.is_null() {
        return Err("Plugin pointer is null".into());
    }
    let plugin_box = unsafe { Box::from_raw(plugin_ptr) };
    Ok(plugin_box)
}