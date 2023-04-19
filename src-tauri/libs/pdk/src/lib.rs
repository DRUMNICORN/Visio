// libs/nodium-pdk/src/lib.rs

pub trait Plugin: Send + Sync {
  fn name(&self) -> String;
  fn nodes(&self) -> Vec<Node>;
  fn services(&self) -> Vec<Service>;
  // Other methods for additional callbacks
}

pub struct Node {
  pub name: String,
  pub description: String,
  // Other fields
}

pub struct Service {
  pub name: String,
  pub description: String,
  pub endpoint: String,
  // Other fields
}
