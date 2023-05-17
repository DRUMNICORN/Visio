use crate::{NodiumWindow, NodiumUiComponent};

pub struct EmptyWindow;

impl NodiumWindow for EmptyWindow {
    fn name(&self) -> String {
        String::new()
    }
    fn icon(&self) -> String {
        String::new()
    }
    fn title(&self) -> String {
        String::new()
    }
    fn content(&self) -> NodiumUiComponent {
        NodiumUiComponent::Text(String::new())
    }
}
