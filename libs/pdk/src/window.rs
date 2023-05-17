use crate::NodiumUiComponent;

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

use std::fmt;

impl fmt::Debug for dyn NodiumWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodiumWindow")
            .field("name", &self.name())
            .field("icon", &self.icon())
            .field("title", &self.title())
            .finish()
    }
}
