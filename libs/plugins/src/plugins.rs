use log::debug;
use nodium_events::EventBus;
use serde_json::Value;
use std::process::Command;
use std::sync::{Arc, Mutex, Weak};

use dirs_next::document_dir;
use std::fs;

use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::Path;

use crate::Registry;

pub struct Plugins {
    install_location: String,
    event_bus: Arc<EventBus>,
    shared_self: Weak<Mutex<Self>>,
    libraries: HashMap<String, Library>,
    registry: Registry,
}

impl Plugins {
    pub fn new(event_bus: Arc<EventBus>) -> Arc<Mutex<Self>> {
        let doc_dir = document_dir().expect("Unable to get user's document directory");
        let install_location = doc_dir.join("nodium").join("plugins");
        if !install_location.exists() {
            fs::create_dir_all(&install_location).expect("Unable to create plugin directory");
        }

        let installer = Arc::new(Mutex::new(Plugins {
            install_location: String::from(install_location.to_str().unwrap()),
            event_bus: event_bus.clone(),
            shared_self: Weak::new(),
            libraries: HashMap::new(),
            registry: Registry::new(),
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
                }) as Box<dyn Fn(String) + Send + Sync>,
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

    fn install_crate(&mut self, crate_name: &str) {
        debug!(
            "Installing crate {} to {}",
            crate_name, self.install_location
        );

        let mut cmd = Command::new("cargo");

        cmd.arg("install")
            .arg("--root")
            .arg(&self.install_location)
            .arg(crate_name);

        let output = cmd
            .output()
            .expect("Failed to execute cargo install command");

        if output.status.success() {
            debug!("Crate {} installed successfully", crate_name);
            self.load_library(crate_name);
        } else {
            debug!("Crate {} failed to install", crate_name);
        }

        debug!(
            "Cargo install output: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    fn load_library(&mut self, crate_name: &str) {
        let lib_path = Path::new(&self.install_location)
            .join("bin")
            .join(crate_name);
        debug!("Loading library from {}", lib_path.to_str().unwrap());
        //  let ext = if cfg!(windows) { "dll" } else if cfg!(unix) { "so" } else { panic!("Unsupported platform"); };

        let lib = unsafe { Library::new(&lib_path) }.expect("Unable to load library");
        debug!("Library loaded successfully");

        // let func: Symbol<unsafe extern "C" fn()> = lib.get(b"function_name").unwrap();

        self.libraries.insert(String::from(crate_name), lib);
        debug!("Library added to libraries map");

        // update registry
        // let plugin: Symbol<unsafe extern "C" fn() -> Box<dyn nodium_pdk::Plugin>> = unsafe { self.libraries.get(crate_name).unwrap().get(b"plugin").unwrap() };
        // let plugin = unsafe { plugin() };
        // self.registry.register_plugin(plugin);
        // debug!("Plugin registered");

        let plugin: Symbol<unsafe extern "C" fn() -> Box<dyn nodium_pdk::Plugin>> = unsafe {
            self.libraries
                .get(crate_name)
                .unwrap()
                .get(b"plugin")
                .unwrap()
        };
        let plugin = unsafe { plugin() };
        self.registry.register_plugin(plugin);
        debug!("Plugin registered");
    }
}
