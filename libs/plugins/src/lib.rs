// libs/plugins/src/plugins.rs
mod plugins;
mod registry;
mod plugin_utils;

// export Plugins and Registry

use nodium_pdk::NodiumPlugin;
pub use plugins::NodiumPlugins;
pub use registry::Registry;

#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    create_plugin: extern "C" fn() -> *mut dyn NodiumPlugin,
}
