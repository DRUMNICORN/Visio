use crate::{Message, Event};

pub trait Tab {
    fn process_event(&self, event: &Event) -> Option<Event>;
    fn title(&self) -> String;
}
