// libs/plugins/src/plugins.rs

pub trait Plugin {
  fn name(&self) -> &'static str;
  fn description(&self) -> &'static str;
  fn version(&self) -> &'static str;
  fn author(&self) -> &'static str;
}