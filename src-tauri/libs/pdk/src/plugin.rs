use std::sync::Arc;

use nodium_events::NodiumEvents;
use tokio::sync::Mutex;

pub trait NodiuimPlugin: Send + Sync {
    fn name(&self) -> String;
    fn nodes(&self, event_bus: Arc<Mutex<NodiumEvents>>) -> Vec<crate::NodiuimNode>;
    fn services(&self, event_bus: Arc<Mutex<NodiumEvents>>) -> Vec<crate::NodiuimService>;
    fn windows(&self, event_bus: Arc<Mutex<NodiumEvents>>) -> Vec<crate::NodiuimWindow>;
}
