// libs/plugins/src/plugins.rs

mod plugin_manager;

pub use plugin_manager::PluginManager;

pub trait Plugin {
  fn name(&self) -> &'static str;
  fn description(&self) -> &'static str;
  fn version(&self) -> &'static str;
  fn author(&self) -> &'static str;
}