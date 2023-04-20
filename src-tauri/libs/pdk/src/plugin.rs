use nodium_events::EventBus;

pub struct Plugin {
  name: String,
  nodes: Vec<crate::Node>,
  services: Vec<crate::Service>,
  windows: Vec<crate::Window>,
  event_bus: EventBus,
}
