use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::debug;

type EventCallback = Box<dyn Fn(String) + Send + Sync>;

pub struct EventBus {
    event_handlers: RwLock<HashMap<String, Vec<EventCallback>>>,
}

impl EventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            event_handlers: RwLock::new(HashMap::new()),
        })
    }

    pub async fn register(&self, event_name: &str, callback: EventCallback) {
        let mut event_handlers = self.event_handlers.write().await;
        let handler_list = event_handlers.entry(event_name.to_string()).or_insert_with(Vec::new);
        handler_list.push(callback);
    }

    pub async fn emit(&self, event_name: &str, payload: String) {
        debug!("Emitting event {}", event_name);
        let event_handlers = self.event_handlers.read().await;
        if let Some(handler_list) = event_handlers.get(event_name) {
            for handler in handler_list {
                handler(payload.clone());
                debug!("Event handler for {} called", event_name);
            }
        }
    }
}
