use crate::{Message, Event};

pub trait Service {
    fn process_message(&self, message: &Message) -> Option<Message>;
}
