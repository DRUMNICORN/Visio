// export Plugins and Registry

use nodium_pdk::DynNodiumPlugin;

mod view;
mod app;
mod plugins;
mod registry;
mod plugin_utils;

pub use app::NodiumApp;
pub use plugins::NodiumPlugins;
pub use registry::Registry;
pub use view::NodiumView;

#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    create_plugin: extern "C" fn() -> DynNodiumPlugin,
}

