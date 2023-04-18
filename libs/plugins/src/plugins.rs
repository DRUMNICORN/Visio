use log::{debug, error, info};
use nodium_events::EventBus;
use serde_json::Value;
use std::process::Command;
use std::sync::{Arc, Weak};
use tar::Archive;

use dirs_next::document_dir;
use std::fs;

use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::Path;

use crate::{extract_crate_file, Registry};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// const for the plugin folder
const PLUGIN_FOLDER: &str = "plugins";

pub struct Plugins {
    install_location: String,
    event_bus: Arc<EventBus>,
    shared_self: Weak<Mutex<Self>>,
    libraries: HashMap<String, Library>,
    registry: Registry,
}

impl Plugins {
    pub async fn new(event_bus: Arc<EventBus>) -> Arc<Mutex<Self>> {
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
        installer.lock().await.shared_self = weak_installer;
        installer
    }

    // reloads all plugins from the plugin folder

    pub async fn reload(&mut self) {
        // go in each folder and load the plugin
        for entry in fs::read_dir(&self.install_location).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut plugin_name = path.file_name().unwrap().to_str().unwrap().to_string();
                plugin_name = plugin_name.split_off(0);
                debug!("Loading plugin {}", plugin_name);
                match self.install(&plugin_name, "", true).await {
                    Ok(_) => {
                        info!("Plugin {} loaded", plugin_name);
                    }
                    Err(e) => {
                        error!("Error loading plugin: {}", e);
                    }
                }

            }
        }
    }

    pub async fn listen(&self) {
        let weak_installer = self.shared_self.clone();
        self.event_bus
            .register(
                "CrateInstall",
                Box::new(move |payload| {
                    debug!("Installing crate {}", payload);
                    if let Some(installer) = weak_installer.upgrade() {
                        let installer = installer.clone();
                        tokio::spawn(async move {
                            let mut installer = installer.lock().await;
                            match installer.download(payload).await {
                                Ok(_) => {
                                    // Handle the success case, e.g., log a success message
                                    info!("Crate downloaded successfully");
                                }
                                Err(e) => {
                                    // Handle the error case, e.g., log an error message
                                    error!("Error downloading crate: {}", e);
                                }
                            }
                        });
                    }
                }) as Box<dyn Fn(String) + Send + Sync>,
            )
            .await;
    }

    async fn download(&mut self, payload: String) -> Result<(), Box<dyn std::error::Error>> {
        let data: Value = serde_json::from_str(&payload).unwrap();
        debug!("Handling event: {}", payload);
        debug!("Event data: {:?}", data);

        let crate_version = data
            .get("crate_version")
            .and_then(Value::as_str)
            .unwrap()
            .to_string();
        let crate_name = data
            .get("crate_name")
            .and_then(Value::as_str)
            .unwrap()
            .to_string();
        let cloned_crate_name = crate_name.clone();
        let cloned_crate_version = crate_version.clone();
        // let mut cloned_self = self.clone(); // Make sure your struct implements Clone

        debug!("Downloading crate {}-{}", crate_name, crate_version);
        match self
            .install(&cloned_crate_name, &cloned_crate_version, false)
            .await
        {
            Ok(_) => {
                info!("Crate {}-{} installed", crate_name, crate_version);
                // self.load_plugin(&cloned_crate_name, &cloned_crate_version).await;
                Ok(())
            }
            Err(e) => {
                error!("Error installing crate: {}", e);
                Err(e)
            }
        }
    }

    pub async fn install(
        &mut self,
        crate_name: &str,
        crate_version: &str,
        lib: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = format!("{}/{}", PLUGIN_FOLDER, crate_name);
        if lib == false {
            let output_dir = format!("{}/{}-{}", PLUGIN_FOLDER, crate_name, crate_version);
        }

        // Create the output directory if it doesn't exist
        fs::create_dir_all(&output_dir)?;

        // Download the crate from crates.io
        let download_url = format!(
            "https://crates.io/api/v1/crates/{}/{}/download",
            crate_name, crate_version
        );
        debug!("Downloading crate from {}", download_url);

        let crate_file_path = format!("{}-{}.crate", crate_name, crate_version);
        extract_crate_file(&crate_file_path, &PLUGIN_FOLDER.to_string()).await?;

        debug!("Building crate {} to {}", crate_name, self.install_location);
        let manifest_path = format!("{}/Cargo.toml", output_dir);

        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(manifest_path);
        let output = cmd.output().expect("Failed to execute cargo build command");
        match !output.status.success() {
            true => {
                debug!("Crate {} failed to build", crate_name);
                error!(
                    "Cargo build output: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            false => {
                debug!("Crate {} built successfully", crate_name);
                self.register(crate_name, crate_version);
            }
        }
        Ok(())
    }

    fn register(&mut self, crate_name: &str, crate_version: &str) {
        let lib_path = Path::new(PLUGIN_FOLDER)
            .join(format!("{}-{}", crate_name, crate_version))
            .join("target")
            .join("release")
            .join(if cfg!(windows) { "lib" } else { "" })
            .join(format!(
                "lib{}{}",
                crate_name,
                if cfg!(windows) {
                    ".dll"
                } else if cfg!(unix) {
                    ".so"
                } else {
                    panic!("Unsupported platform");
                }
            ));

        debug!("Loading library from {}", lib_path.to_str().unwrap());

        let lib = unsafe { Library::new(&lib_path) }.expect("Unable to load library");
        debug!("Library loaded successfully");

        // let func: Symbol<unsafe extern "C" fn()> = lib.get(b"function_name").unwrap();

        self.libraries.insert(String::from(crate_name), lib);
        debug!("Library added to libraries map");

        let plugin: Symbol<unsafe extern "C" fn() -> Box<dyn nodium_pdk::Plugin>> = unsafe {
            self.libraries
                .get(crate_name)
                .unwrap()
                .get(b"plugin")
                .unwrap()
        };

        let plugin = unsafe { plugin() };
        let plugin_name = plugin.name();
        self.registry.register_plugin(plugin);
        debug!("Plugin {} registered", plugin_name);
    }
}
