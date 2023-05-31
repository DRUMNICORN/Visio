// libs/plugins/src/plugins.rs
mod plugins;
mod registry;
mod extract_crate_file;
mod plugin_utils;

// export Plugins and Registry

use nodium_pdk::NodiumPlugin;
pub use plugins::NodiumPlugins;
pub use registry::Registry;
pub use extract_crate_file::extract_crate_file;

#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    create_plugin: extern "C" fn() -> *mut dyn NodiumPlugin,
}
