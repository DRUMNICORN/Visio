use dlopen::wrapper::Container;
use nodium_pdk::{DynNodiumNode, DynNodiumPlugin};

use crate::utils::{create_plugins_directory, get_lib_path, install, rebuild};
use crate::PluginApi;
use std::path::PathBuf;
use std::str;
// use dirs_next::document_dir;
use log::{debug, error, info, warn};

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NodiumRegistry {
    install_location: String,
    plugins: Arc<Mutex<HashMap<String, Arc<DynNodiumPlugin>>>>,
    nodes: Arc<Mutex<HashMap<String, Arc<DynNodiumNode>>>>,
}

impl Debug for NodiumRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodiumRegistry")
            .field("install_location", &self.install_location)
            .finish()
    }
}

impl NodiumRegistry {
    pub fn new() -> Self {
        //       let doc_dir = document_dir().expect("Unable to get user's document directory");
        //let install_location = "/home/roggen/Documents/GitHub/nodium/plugins".to_string();

        // use fixxed location in current dir inside plugins
        let install_location = PathBuf::from("/home/roggen/Documents/GitHub/nodium/plugins/");

        debug!("Plugin install location: {:?}", install_location);
        if !install_location.exists() {
            debug!("Creating plugin directory");
            fs::create_dir_all(&install_location).expect("Unable to create plugin directory");
        }
        let registry = NodiumRegistry {
            install_location: install_location.to_string_lossy().to_string(),
            plugins: Arc::new(Mutex::new(HashMap::new())),
            nodes: Arc::new(Mutex::new(HashMap::new())),
        };

        registry
    }

    pub async fn rebuild(&mut self) {
        //        self.unregister_all().await; // TODO
        rebuild(&self.install_location).await.unwrap_or_else(|e| {
            error!("Error rebuilding plugins: {}", e);
        });

        // Call the reload function after rebuilding
        self.reload().await;
    }

    pub async fn reload(&mut self) {
        debug!("Reloading plugins");
        let plugins_dir = Path::new(&self.install_location);
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

                    if let Err(e) = self.register_plugin_and_nodes(plugin_name, true).await {
                        warn!("Plugin not able to register: {}", e);
                        if let Err(e) = install(
                            path.file_name().unwrap().to_str().unwrap(),
                            "",
                            &self.install_location,
                            true,
                        )
                        .await
                        {
                            error!("Error installing plugin: {}", e);
                        } else if let Err(e) =
                            self.register_plugin_and_nodes(plugin_name, true).await
                        {
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

    pub async fn listen(&self, registry: Arc<Mutex<Self>>) {
        let _registry_clone = registry.clone();
        // TODO: Load plugins in the plugins directory
        //todo!("Load plugins in the plugins directory");
    }

    async fn register_plugin_and_nodes(
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
        
        let plugin_arc = Arc::new(plugin);
        self.plugins
            .lock()
            .await
            .insert(plugin_name.clone().to_owned(), plugin_arc.clone());
        
        // let plugin_nodes = plugin.nodes();
        // plugin_nodes.iter().into_iter().for_each(|node| {
        //     let node_name = str::from_utf8(node.name().as_bytes())
        //         .unwrap_or_else(|e| {
        //             panic!("Invalid UTF-8 sequence in node name: {:?}", e);
        //         })
        //         .to_owned();
        //     debug!("Registering node: {}", node_name);
        //     self.nodes.lock().await.insert(node_name, Arc::new((*node).clone()));
        // });

        Ok(())
    }

    pub async fn get_plugins(&self) -> Vec<String> {
        self.plugins.lock().await.keys().cloned().collect()
    }

    pub async fn get_nodes(&self) -> Vec<String> {
        self.nodes.lock().await.keys().cloned().collect()
    }
}
