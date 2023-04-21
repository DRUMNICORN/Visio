use crate::plugin_utils::{download, install};
use crate::Registry;
use dirs_next::document_dir;
use libloading::{Library, Symbol};
use log::{debug, error, info};
use nodium_events::{NodiumEventType, NodiumEvents};
use nodium_pdk::NodiuimPlugin;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NodiumPlugins {
    install_location: String,
    event_bus: Arc<Mutex<NodiumEvents>>,
    registry: Registry,
}

impl NodiumPlugins {
    pub async fn new(event_bus: Arc<Mutex<NodiumEvents>>) -> Arc<Mutex<Self>> {
        let doc_dir = document_dir().expect("Unable to get user's document directory");
        let install_location = doc_dir.join("nodium").join("plugins");
        debug!("Plugin install location: {:?}", install_location);
        if !install_location.exists() {
            debug!("Creating plugin directory");
            fs::create_dir_all(&install_location).expect("Unable to create plugin directory");
        }
        let installer = Arc::new(Mutex::new(NodiumPlugins {
            install_location: install_location.to_str().unwrap().to_string(),
            event_bus: event_bus.clone(),
            registry: Registry::new(),
        }));

        // load plugins in the plugins directory
        installer.lock().await.reload().await;
        installer
    }

    pub async fn reload(&mut self) {
        debug!("Reloading plugins");
        // load plugins in the plugins directory
        let plugins_dir = Path::new(&self.install_location);
        if !plugins_dir.exists() {
            debug!("Plugins directory does not exist");
            // create plugins directory
            match fs::create_dir_all(&plugins_dir) {
                Ok(_) => {
                    debug!("Plugins directory created successfully");
                }
                Err(e) => {
                    error!("Error creating plugins directory: {}", e);
                    return;
                }
            }
        }
        let folders = match fs::read_dir(&plugins_dir) {
            Ok(folders) => folders,
            Err(e) => {
                error!("Error reading plugins directory: {}", e);
                return;
            }
        };

        for entry in folders {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    error!("Error reading plugin directory: {}", e);
                    continue;
                }
            };
            let path = entry.path();
            debug!("Plugin path: {:?}", path);
            if path.is_dir() {
                let plugin_name = path.file_name().unwrap().to_str().unwrap();
                let plugin_version = path.file_name().unwrap().to_str().unwrap();
                debug!(
                    "Plugin name and version: {} {}",
                    plugin_name, plugin_version
                );
                match self.register(plugin_name, plugin_version, true) {
                    Ok(_) => {
                        info!("Plugin registered successfully");
                    }
                    Err(e) => {
                        info!("Error registering plugin: {} ... trying to build", e);
                        // get folder name of last folder in path
                        match install(
                            path.file_name().unwrap().to_str().unwrap(),
                            "",
                            &self.install_location,
                            true,
                        )
                        .await
                        {
                            Ok(_) => {
                                info!("Plugin installed successfully");
                            }
                            Err(e) => {
                                error!("Error installing plugin: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn listen(this: Arc<Mutex<Self>>) {
        let plugins_clone = this.clone();
        let plugins_clone_callback = plugins_clone.clone();

        let event_bus_guard = plugins_clone.lock().await.event_bus.clone();

        event_bus_guard
            .lock()
            .await
            .register(
                &NodiumEventType::AddPlugin.to_string(),
                Box::new(move |payload: String| {
                    let plugins_clone = plugins_clone_callback.clone();

                    tokio::spawn(async move {
                        let mut plugins_guard = plugins_clone.lock().await;
                        let install_location = plugins_guard.install_location.clone();
                        match plugins_guard
                            .plugin_install(payload, install_location.clone())
                            .await
                        {
                            Ok((crate_name, crate_version)) => {
                                let mut plugins_guard = plugins_clone.lock().await;
                                match plugins_guard.register(&crate_name, &crate_version, false) {
                                    Ok(_) => {
                                        info!("Plugin registered successfully");
                                    }
                                    Err(e) => {
                                        error!("Error registering plugin: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error installing crate: {}", e);
                            }
                        }
                    });
                }) as Box<dyn Fn(String) + Send + Sync>,
            )
            .await;
    }

    async fn plugin_install(
        &mut self,
        payload: String,
        install_location: String,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        let install_location = install_location.clone();
        debug!("Installing crate {}", payload);
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

        match download(&crate_name, &crate_version, &install_location).await {
            Ok(_) => {
                info!("Crate downloaded successfully");
                match install(&crate_name, &crate_version, &install_location, false).await {
                    Ok(_) => {
                        info!("Crate installed successfully");
                        Ok((crate_name, crate_version))
                    }
                    Err(e) => {
                        error!("Error installing crate: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Error downloading crate: {}", e);
                Err(e)
            }
        }
    }

    fn register(
        &mut self,
        crate_name: &str,
        crate_version: &str,
        is_local: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let folder_name = if is_local {
            crate_name.to_string()
        } else {
            format!("{}-{}", crate_name, crate_version)
        };
        let lib_path = Path::new(&self.install_location)
            .join(folder_name)
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
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unsupported platform",
                    )));
                }
            ));

        let lib = unsafe { Library::new(&lib_path) }?;
        let plugin: Symbol<unsafe extern "C" fn() -> Box<dyn NodiuimPlugin>> =
            unsafe { lib.get(b"plugin")? };

        let plugin = unsafe { plugin() };
        let plugin_name = plugin.name();
        debug!("Registering plugin: {}", plugin_name);
        let event_bus = self.event_bus.clone();
        self.registry.register_plugin(event_bus, plugin);
        Ok(())
    }
}
