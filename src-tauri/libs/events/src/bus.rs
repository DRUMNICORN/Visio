use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use log::debug;

type EventCallback = Box<dyn Fn(String) + Send + Sync>;
type EventNotifier = Box<dyn Fn(&str, &str) + Send + Sync>;


pub struct NodiumEventBus {
  event_handlers: RwLock<HashMap<String, Vec<EventCallback>>>,
  event_notifier: EventNotifier,
}

impl NodiumEventBus {
  pub fn new(event_notifier: EventNotifier) -> Arc<Mutex<Self>> {
      Arc::new(Mutex::new(Self {
          event_handlers: RwLock::new(HashMap::new()),
          event_notifier,
      }))
  }

  pub async fn register(&self, event_name: &str, callback: EventCallback) {
    debug!("Registering event {}", event_name);
      let mut event_handlers = self.event_handlers.write().await;
      let handler_list = event_handlers.entry(event_name.to_string()).or_insert_with(Vec::new);
      handler_list.push(callback);
  }

  pub async fn emit(&self, event_name: &str, payload: String) {
      debug!("Emitting event {}", event_name);
      let event_handlers = self.event_handlers.read().await;
      if let Some(handler_list) = event_handlers.get(event_name) {
          debug!("Found {} handlers for event {}", handler_list.len(), event_name);
          for handler in handler_list {
              let handler = handler.clone();
              handler(payload.clone());
          }
          debug!("Finished emitting event {}", event_name);
      }
      (self.event_notifier)(event_name, &payload);
  }

  pub fn send(&self, event_name: &str, payload: String) {
      debug!("Sending event {}", event_name);
      (self.event_notifier)(event_name, &payload);
  }
}
