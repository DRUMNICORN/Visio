use crate::utils::{
    download, extract_crate_version_name, extract_folder_name, extract_plugin, get_lib_path,
    install,
};
use crate::Registry;
use dirs_next::document_dir;
use libloading::{Library};
use log::{debug, error, info, warn};
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NodiumPlugins {
    install_location: String,
    registry: Registry,
}

impl Debug for NodiumPlugins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodiumPlugins")
            .field("install_location", &self.install_location)
            .finish()
    }
}

impl NodiumPlugins {
    pub fn new() -> Arc<Mutex<Self>> {
        let _doc_dir = document_dir().expect("Unable to get user's document directory");
        // let install_location = doc_dir.join("nodium").join("plugins");
        let install_location_str = "/home/roggen/Repos/nodium/plugins";
        let install_location = Path::new(install_location_str);
        // print absolute path to plugins directory
        let absolute_path = fs::canonicalize(&install_location)
            .expect("Unable to get absolute path to plugins directory");
        debug!("Absolute path to plugins directory: {:?}", absolute_path);

        debug!("Plugin install location: {:?}", install_location);
        if !install_location.exists() {
            debug!("Creating plugin directory");
            fs::create_dir_all(&install_location).expect("Unable to create plugin directory");
        }
        let plugins = NodiumPlugins {
            install_location: install_location_str.to_string(),
            registry: Registry::new(),
        };

        let plugins = Arc::new(Mutex::new(plugins));
        plugins
    }

    pub async fn reload(&mut self) {
        debug!("Reloading plugins");

        let plugins_dir = Path::new(&self.install_location);
        if !plugins_dir.exists() {
            debug!("Plugins directory does not exist");
            if let Err(e) = fs::create_dir_all(&plugins_dir) {
                error!("Error creating plugins directory: {}", e);
                return;
            }
            debug!("Plugins directory created successfully");
        }

        debug!("Reading plugins directory");
        match fs::read_dir(&plugins_dir) {
            Ok(folders) => {
                debug!("Plugins directory read successfully");
                for entry in folders {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        debug!("Plugin path: {:?}", path);
                        if path.is_dir() {
                            let plugin_name = path.file_name().unwrap().to_str().unwrap();
                            debug!(
                                "Plugin name: {}",
                                plugin_name
                            );
                            if let Ok(_) = self.register(plugin_name, true) {
                                info!("Plugin registered successfully");
                            } else {
                                warn!("Plugin not able to register");
                                if let Err(e) = install(
                                    path.file_name().unwrap().to_str().unwrap(),
                                    "",
                                    &self.install_location,
                                    true,
                                )
                                .await
                                {
                                    error!("Error installing plugin: {}", e);
                                    continue;
                                }
                                if let Ok(_) = self.register(plugin_name, true) {
                                    info!("Plugin registered successfully");
                                } else {
                                    error!("Error registering plugin");
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error reading plugins directory: {}", e);
            }
        }
    }

    pub fn listen(&self, plugins: Arc<Mutex<Self>>) {
        let plugins_clone = plugins.clone();
        let _plugins_clone_callback = plugins_clone.clone();
        // TODO: load plugins in the plugins directory
        todo!("load plugins in the plugins directory");
    }

    async fn _plugin_install(
        &mut self,
        payload: String,
        install_location: String,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        let install_location = install_location.clone();
        debug!("Installing crate {}", payload);
        let (crate_version, crate_name) = extract_crate_version_name(payload);

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
        is_local: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Registering plugin {}", crate_name);
        let folder_name = extract_folder_name(is_local, crate_name);
        let lib_path = get_lib_path(&self.install_location, folder_name, crate_name)?;

        let lib = unsafe { Library::new(lib_path) };

        match lib {
            Ok(lib) => match extract_plugin(lib) {
                Ok(plugin) => {
                    debug!("Plugin extracted successfully");
                    let plugin = self.registry.register_plugin(plugin);
                    match plugin {
                        Some(plugin) => {
                            debug!("Plugin registered successfully");
                            let plugin_name = plugin.name();
                            let plugin_version = plugin.version();
                            info!("Plugin registered: {} {}", plugin_name, plugin_version);
                            Ok(())
                        }
                        None => {
                            error!("Error registering plugin");
                            let error = format!("Error registering plugin");
                            Err(error.into())
                        }
                    }
                }
                Err(e) => {
                    error!("Error extracting plugin: {}", e);
                    let error = format!("Error extracting plugin: {}", e);
                    Err(error.into())
                }
            },
            Err(e) => {
                error!("Error loading plugin: {}", e);
                Err(e.into())
            }
        }
    }

    pub fn get_plugins(&self) -> Vec<String> {
        debug!("Getting plugins");
        let plugins = self.registry.get_plugins();
        debug!("Plugins: {:?}", plugins);
        plugins
    }
}
