use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

type EventCallback = Box<dyn Fn(&str) + Send + Sync>;

pub struct NodiumEventBus {
    event_handlers: RwLock<HashMap<String, Vec<EventCallback>>>,
}

impl NodiumEventBus {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            event_handlers: RwLock::new(HashMap::new()),
        }))
    }

    pub async fn register(&self, event_name: &str, callback: EventCallback) {
        debug!("Registering event {}", event_name);
        let mut event_handlers = self.event_handlers.write().await;
        let handler_list = event_handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new);
        handler_list.push(callback);
    }

    pub async fn emit(&self, event_name: &str, payload: String) {
        debug!("Emitting event {}", event_name);
        let event_handlers = self.event_handlers.read().await;
        if let Some(handler_list) = event_handlers.get(event_name) {
            debug!(
                "Found {} handlers for event {}",
                handler_list.len(),
                event_name
            );
            for handler in handler_list {
                let handler = handler.clone();
                handler(&payload);
            }
            debug!("Finished emitting event {}", event_name);
        }
    }
}
