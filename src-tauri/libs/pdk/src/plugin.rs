use std::sync::Arc;
use tokio::sync::Mutex;

use nodium_events::NodiumEventBus;

pub trait NodiumPlugin: Send + Sync {
  fn name(&self) -> String;
  fn nodes(&self, event_bus: Arc<Mutex<NodiumEventBus>>) -> Vec<crate::NodiumNode>;
  fn services(&self, event_bus: Arc<Mutex<NodiumEventBus>>) -> Vec<crate::NodiumService>;
  fn windows(&self, event_bus: Arc<Mutex<NodiumEventBus>>) -> Vec<Box<dyn crate::NodiumWindow>>;
  fn with_event_bus(&mut self, event_bus: NodiumEventBus);
}

