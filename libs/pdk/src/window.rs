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

pub trait NodiumWindow: Send + Sync {
    // TODO: NodiumWindow should be a generator for a UI component, there are many ways to display a UI component, so this trait should be renamed to NodiumUiComponentGenerator
    // TODO: there need to be a way to pass data to the UI component, so the UI component can be dynamic and not static
    fn name(&self) -> String;
    fn icon(&self) -> String;
    fn title(&self) -> String;
    fn content(&self) -> NodiumUiComponent;
    // fn on_event(&mut self, event_name: &str, _event_data: &str);

    fn serialize(&self) -> String {
        serde_json::to_string(&serde_json::json!({
          "name": self.name(),
          "icon": self.icon(),
          "title": self.title(),
          "content": self.content(),
        }))
        .unwrap()
    }
}
