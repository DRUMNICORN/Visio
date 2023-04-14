use std::collections::HashMap;

use crate::core::node_type::NodeType;

pub struct Node {
  id: String,
  node_type: NodeType,
  properties: HashMap<String, String>,
}

impl Node {
  pub fn new(id: String, node_type: NodeType, properties: HashMap<String, String>) -> Self {
      Self {
          id,
          node_type,
          properties,
      }
  }

  pub fn get_id(&self) -> &str {
      &self.id
  }

  pub fn get_node_type(&self) -> &NodeType {
      &self.node_type
  }

  pub fn get_property(&self, key: &str) -> Option<&String> {
      self.properties.get(key)
  }

  pub fn set_property(&mut self, key: String, value: String) {
      self.properties.insert(key, value);
  }

  pub fn remove_property(&mut self, key: &str) {
      self.properties.remove(key);
  }
}
