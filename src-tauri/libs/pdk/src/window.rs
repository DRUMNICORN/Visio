
use serde::{Serialize, Serializer, ser::SerializeStruct};
// copy clone

#[derive(Clone, Debug)]
pub struct NodiuimWindow {
  pub name: String,
  pub icon: String,
  pub title: String,
  pub content: String,
}

// to value
impl Serialize for NodiuimWindow {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("NodiuimWindow", 4)?;
    state.serialize_field("name", &self.name)?;
    state.serialize_field("icon", &self.icon)?;
    state.serialize_field("title", &self.title)?;
    state.serialize_field("content", &self.content)?;
    state.end()
  }
}