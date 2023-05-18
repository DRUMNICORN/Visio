pub use crate::registry::RegistryPlugin;
use crate::utils::{
    build, extract_folder_name, extract_plugin, get_lib_path,
};
use crate::Registry;
use log::{debug, error, info, warn};
use std::{fmt::Debug, fs, path::Path, sync::Arc};
use tokio::sync::Mutex;

pub struct NodiumPlugins {
    install_location: Box<Path>,
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
        let install_location_str = "/home/roggen/Repos/nodium/plugins";
        let install_location = Path::new(install_location_str);
        let absolute_path = fs::canonicalize(&install_location)
            .expect("Unable to get absolute path to plugins directory");
        debug!("Absolute path to plugins directory: {:?}", absolute_path);

        debug!("Plugin install location: {:?}", install_location);
        if !install_location.exists() {
            debug!("Creating plugin directory");
            fs::create_dir_all(&install_location).expect("Unable to create plugin directory");
        }

        let plugins = NodiumPlugins {
            install_location: Box::from(install_location),
            registry: Registry::new(),
        };

        let plugins = Arc::new(Mutex::new(plugins));
        plugins
    }

    pub async fn reload(&mut self) {
        debug!("Reloading plugins");
        let unboxed_install_location = self.install_location.to_str().unwrap().to_string();
        let plugins_dir = Path::new(&unboxed_install_location);
        if !plugins_dir.exists() {
            debug!("Plugins directory does not exist");
            if let Err(e) = fs::create_dir_all(&plugins_dir) {
                error!("Error creating plugins directory: {}", e);
                return;
            }
            debug!("Plugins directory created successfully");
        }

        if let Err(e) = self.read_plugins_directory(&plugins_dir).await {
            error!("Error reading plugins directory: {}", e);
            return;
        }

        debug!("Plugins directory read successfully");
    }

    async fn read_plugins_directory(
        &mut self,
        plugins_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let folders = fs::read_dir(&plugins_dir)?;
        for entry in folders {
            if let Ok(entry) = entry {
                let path = entry.path();
                debug!("Plugin path: {:?}", path);
                if path.is_dir() {
                    let plugin_name = path.file_name().unwrap().to_str().unwrap();
                    debug!("Plugin name: {}", plugin_name);
                    if let Ok(_) = self.register(plugin_name, true) {
                        info!("Plugin registered successfully");
                    } else {
                        if let Err(e) = self.build_and_register_plugin(&path, plugin_name).await {
                            error!("Error installing plugin: {}", e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn build_and_register_plugin(
        &mut self,
        path: &Path,
        plugin_name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path = path.to_str().unwrap();
        let builded = build(plugin_name, path).await;
        if let Err(e) = builded {
            error!("Error building plugin: {}", e);
            return Err(e.into());
        }
        if let Ok(_) = self.register(plugin_name, true) {
            info!("Plugin registered successfully");
            Ok(())
        } else {
            error!("Error registering plugin");
            Err("Error registering plugin".into())
        }
    }

    pub fn listen(&self, plugins: Arc<Mutex<Self>>) {
        let plugins_clone = plugins.clone();
        let _plugins_clone_callback = plugins_clone.clone();
        // TODO: load plugins in the plugins directory
        todo!("load plugins in the plugins directory");
    }

    fn register(
        &mut self,
        crate_name: &str,
        is_local: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Registering plugin {}", crate_name);
        let folder_name = extract_folder_name(is_local, crate_name);
        let unboxed_install_location = self.install_location.to_str().unwrap();
        let lib_path = get_lib_path(unboxed_install_location, folder_name, crate_name)?;
        let lib_path = Path::new(&lib_path);
        let plugin_result = unsafe { extract_plugin(&lib_path) };
        match plugin_result {
            Ok(plugin) => {
                let registerd_plugin = self.registry.register_plugin(RegistryPlugin {
                    name: plugin.name().to_string(),
                    version: plugin.version().to_string(),
                });
                match registerd_plugin {
                    Ok(_) => {
                        info!("Plugin registered successfully");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Error registering plugin: {}", e);
                        return Err(e.into());
                    }
                }
            }
            Err(e) => {
                warn!("Error extracting plugin: {}", e);
                let error = format!("Error extracting plugin: {}", e);
                Err(error.into())
            }
        }
    }

    pub fn get_plugins(&self) -> Vec<&RegistryPlugin> {
        self.registry.get_plugins()
    }
}
