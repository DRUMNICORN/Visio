// libs/plugins/src/plugins.rs
mod plugins;
mod registry;
mod download;

// export Plugins and Registry

pub use plugins::Plugins;
pub use registry::Registry;
pub use download::download_crate;