use crate::utils::{install, create_plugins_directory, rebuild, get_lib_path};
use dirs_next::document_dir;
use dlopen::wrapper::Container;
use log::{debug, error, info, warn};
use nodium_pdk::DynNodiumPlugin;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use std::collections::HashMap;


use crate::PluginApi;

pub struct NodiumPlugins {
    install_location: String,
    plugins: HashMap<String, DynNodiumPlugin>,
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
        let doc_dir = document_dir().expect("Unable to get user's document directory");
        let install_location = doc_dir.join("nodium").join("plugins");
        debug!("Plugin install location: {:?}", install_location);
        if !install_location.exists() {
            debug!("Creating plugin directory");
            fs::create_dir_all(&install_location)
                .expect("Unable to create plugin directory");
        }
        let plugins = NodiumPlugins {
            install_location: "plugins".to_string(),
            plugins: HashMap::new(),
        };

        let installer = Arc::new(Mutex::new(plugins));
        let installer_clone = installer.clone();
        tokio::spawn(async move {
            installer_clone.lock().await.listen(installer_clone.clone()).await;
        });
        installer
    }

    pub async fn rebuild(&mut self) {
        self.unregister_all().await;
        rebuild(&self.install_location)
            .await
            .unwrap_or_else(|e| {
                error!("Error rebuilding plugins: {}", e);
            });

        // Call the reload function after rebuilding
        self.reload().await;
    }

    pub async fn reload(&mut self) {
        let install_location = self.install_location.clone();
        debug!("Reloading plugins");
        let plugins_dir = Path::new(&install_location);
        create_plugins_directory(&plugins_dir).unwrap_or_else(|e| {
            error!("Error creating plugins directory: {}", e);
            return;
        });
    
        if let Ok(folders) = fs::read_dir(&plugins_dir) {
            for entry in folders.filter_map(Result::ok) {
                let path = entry.path();
                debug!("Plugin path: {:?}", path);
                if path.is_dir() {
                    let plugin_name = path.file_name().unwrap().to_str().unwrap();

                    if let Err(e) = self.register(plugin_name, true) {
                        warn!("Plugin not able to register: {}", e);
                        if
                            let Err(e) = install(
                                path.file_name().unwrap().to_str().unwrap(),
                                "",
                                &self.install_location,
                                true
                            ).await
                        {
                            error!("Error installing plugin: {}", e);
                        } else if let Err(e) = self.register(plugin_name, true) {
                            error!("Error registering plugin: {}", e);
                        } else {
                            info!("Plugin registered successfully");
                        }
                    } else {
                        info!("Plugin registered successfully");
                    }
                }
            }
        } else {
            error!("Error reading plugins directory");
        }
    }
    
    

    async fn unregister_all(&mut self) {
        self.plugins.clear();
    }

    pub async fn listen(&self, plugins: Arc<Mutex<Self>>) {
        let plugins_clone = plugins.clone();
        let _plugins_clone_callback = plugins_clone.clone();
        // TODO: load plugins in the plugins directory
        //todo!("Load plugins in the plugins directory");
    }

    pub fn register(
        &mut self,
        crate_name: &str,
        is_local: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let folder_name = if is_local {
            crate_name.to_string()
        } else {
            format!("{}", crate_name)
        };
    
        let lib_path = get_lib_path(&self.install_location, &folder_name, crate_name)?;
    
        // Load the plugin using the dlopen crate.
        let plugin_api_wrapper: Container<PluginApi> = (unsafe { Container::load(lib_path) })?;
    
        // Load the plugin using the dlopen crate.
        let plugin: DynNodiumPlugin = (plugin_api_wrapper.create_plugin)();
        let plugin_name = plugin.name();
        debug!("Registering plugin: {}", plugin_name);
    
        self.plugins.insert(plugin_name.clone().to_owned(), plugin);
    
        Ok(())
    }
    

    pub fn get_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
