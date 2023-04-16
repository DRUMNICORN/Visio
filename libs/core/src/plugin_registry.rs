// libs/core/src/plugin_registry.rs

pub struct PluginRegistry {
  plugins: Vec<PluginInfo>,
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
  pub name: String,
  pub version: String,
  pub repository: String,
}

impl PluginRegistry {
  pub fn new() -> Self {
      PluginRegistry { plugins: Vec::new() }
  }

  pub fn add_plugin(&mut self, plugin_info: PluginInfo) {
      self.plugins.push(plugin_info);
  }

  pub fn get_plugins(&self) -> &[PluginInfo] {
      &self.plugins
  }
}
