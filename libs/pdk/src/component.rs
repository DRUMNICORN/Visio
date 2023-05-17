#[derive(Clone, Debug, serde::Serialize)]
pub enum NodiumUiComponent {
  Text(String),
  InputField(String, String), // (label, event_name)
  Button(String, String),     // (label, event_name)
  List(Vec<String>),
  Table(Vec<Vec<String>>),
  Tree(Vec<NodiumUiComponent>),
  Tabs(Vec<NodiumUiComponent>),
}
