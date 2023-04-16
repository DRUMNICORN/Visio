// libs/core/src/crate_installer.rs
use log::debug;
use nodium_events::EventBus;
use serde_json::Value;
use std::process::Command;
use std::sync::{Arc, Mutex, Weak};

#[derive(Clone)]
pub struct CrateInstaller {
    event_bus: Arc<EventBus>,
    shared_self: Weak<Mutex<Self>>,
}

impl CrateInstaller {
    pub fn new(event_bus: Arc<EventBus>) -> Arc<Mutex<Self>> {
        let installer = Arc::new(Mutex::new(CrateInstaller {
            event_bus: event_bus.clone(),
            shared_self: Weak::new(),
        }));
        let weak_installer = Arc::downgrade(&installer);
        installer.lock().unwrap().shared_self = weak_installer;
        installer
    }

    pub async fn register_event_handlers(&self) {
        let weak_installer = self.shared_self.clone();
        self.event_bus
            .register(
                "CrateInstall",
                Box::new(move |payload| {
                    if let Some(installer) = weak_installer.upgrade() {
                        let mut installer = installer.lock().unwrap();
                        installer.handle_event(payload);
                        debug!("CrateInstall event handler called");
                    }
                }),
            )
            .await;
    }

    fn handle_event(&mut self, payload: String) {
        let data: Value = serde_json::from_str(&payload).unwrap();
        if let Some(crate_name) = data.get("crate_name").and_then(Value::as_str) {
            debug!("Handling CrateInstall event for {}", crate_name);
            self.install_crate(crate_name);
        }
        // Handle other event types as needed
    }

    fn install_crate(&self, crate_name: &str) {
        debug!("Installing crate {}", crate_name);
        let status = Command::new("cargo")
            .arg("install")
            .arg(crate_name)
            .status()
            .expect("Failed to execute cargo install command");

        if status.success() {
            debug!("Crate {} installed successfully", crate_name);
        } else {
            debug!("Failed to install crate {}", crate_name);
        }
    }
}
