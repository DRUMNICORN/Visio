// Export Plugins and Registry

use nodium_pdk::DynNodiumPlugin;

mod view;
mod app;
mod registry;
mod utils;
mod flow;

pub use app::NodiumApp;
pub use registry::NodiumRegistry;
pub use view::NodiumView;

#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    create_plugin: extern "C" fn() -> DynNodiumPlugin,
}
