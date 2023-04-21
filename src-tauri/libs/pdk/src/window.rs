
use serde::{Serialize,  Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodiuimWindow {
  pub name: String,
  pub icon: String,
  pub title: String,
  pub content: NodiumUiComponent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodiumUiComponent {
  Text(String),
  InputField(String, String), // (label, event_name)
  Button(String, String), // (label, event_name)
  List(Vec<String>),
  Table(Vec<Vec<String>>),
  Tree(Vec<NodiumUiComponent>),
  Tabs(Vec<NodiumUiComponent>),
}
