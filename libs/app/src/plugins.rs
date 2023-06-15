use crate::plugin_utils::{ install };
use crate::{ Registry, PluginApi };
use dirs_next::document_dir;
use dlopen::wrapper::Container;
use log::{ debug, error, info, warn };
use nodium_pdk::DynNodiumPlugin;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::plugin_utils::create_plugins_directory;
use crate::plugin_utils::rebuild;

pub struct NodiumPlugins {
    install_location: String,
    registry: Registry,
}

impl Debug for NodiumPlugins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodiumPlugins").field("install_location", &self.install_location).finish()
    }
}

impl NodiumPlugins {
    pub fn new() -> Arc<Mutex<Self>> {
        let doc_dir = document_dir().expect("Unable to get user's document directory");
        let install_location = doc_dir.join("nodium").join("plugins");
        debug!("Plugin install location: {:?}", install_location);
        if !install_location.exists() {
            debug!("Creating plugin directory");
            fs::create_dir_all(&install_location).expect("Unable to create plugin directory");
        }
        let plugins = NodiumPlugins {
            install_location: "plugins".to_string(),
            registry: Registry::new(),
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
                    let plugin_version = path.file_name().unwrap().to_str().unwrap();
                    debug!("Plugin name and version: {} {}", plugin_name, plugin_version);

                    if let Err(e) = self.register(plugin_name, plugin_version, true) {
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
                        } else if let Err(e) = self.register(plugin_name, plugin_version, true) {
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
        self.registry.unregister_all_plugins();
    } 

    pub async fn listen(&self, plugins: Arc<Mutex<Self>>) {
        let plugins_clone = plugins.clone();
        let _plugins_clone_callback = plugins_clone.clone();
        // TODO: load plugins in the plugins directory
        todo!("Load plugins in the plugins directory");
    }

    pub fn register(
        &mut self,
        crate_name: &str,
        crate_version: &str,
        is_local: bool
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
            .join(
                format!("lib{}{}", crate_name, if cfg!(windows) {
                    ".dll"
                } else if cfg!(unix) {
                    ".so"
                } else {
                    // Todo: add support for other platforms (macos, ios, android, etc.)
                    return Err(
                        Box::new(
                            std::io::Error::new(std::io::ErrorKind::Other, "Unsupported platform")
                        )
                    );
                })
            );

        // TODO: Check out: abi_stable and Foreign Function Interface

        // Load the plugin using the dlopen crate.
        let plugin_api_wrapper: Container<PluginApi> = (unsafe { Container::load(lib_path) })?;

        // Load the plugin using the dlopen crate.
        let plugin: DynNodiumPlugin = unsafe {
            (plugin_api_wrapper.create_plugin)()
        };
        let plugin_name = plugin.name();
        debug!("Registering plugin: {}", plugin_name);
        
        self.registry.register_plugin(plugin);

        Ok(())
    }

    pub fn get_plugins(&self) -> Vec<String> {
        let plugins = self.registry.get_plugins();
        plugins
    }
}
