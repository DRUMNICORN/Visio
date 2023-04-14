use crate::{Message, Event};

pub trait Node {
    fn process_event(&self, event: &Event) -> Option<Event>;
    fn inputs(&self) -> Vec<String>;
    fn outputs(&self) -> Vec<String>;
}
