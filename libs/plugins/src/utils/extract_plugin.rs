use libloading::{Library, Symbol};
use nodium_pdk::NodiumPluginObject;

pub fn extract_plugin(lib: Library) -> Result<Box<dyn NodiumPluginObject>, Box<dyn std::error::Error>> {
  let plugin: Symbol<unsafe extern "C" fn() -> Box<dyn NodiumPluginObject>> =
      unsafe { lib.get(b"plugin")? };
  let plugin = unsafe { plugin() };
  Ok(plugin)
}