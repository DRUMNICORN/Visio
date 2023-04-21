#[derive(Debug, Clone)]
pub struct NodiuimNode {
  pub name: String,
  pub description: String,
  // Other fields
}

use serde::{Serialize, Serializer, ser::SerializeStruct};

impl Serialize for 
NodiuimNode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("NodiuimNode", 2)?;
    state.serialize_field("name", &self.name)?;
    state.serialize_field("description", &self.description)?;
    state.end()
  }
}
