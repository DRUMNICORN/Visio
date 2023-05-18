// libs/plugins/src/plugins.rs
mod plugins;
mod registry;
mod extract_crate_file;
mod utils;

// export Plugins and Registry

pub use registry::RegistryPlugin;
pub use plugins::NodiumPlugins;
pub use registry::Registry;
pub use extract_crate_file::extract_crate_file;