// nodium_view_console.rs
use log::{debug};
use nodium_app::{NodiumApp};
use std::{
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct NodiumViewConsole {
    app: Arc<Mutex<NodiumApp>>,
}

impl NodiumViewConsole {
    pub fn new(app: Arc<Mutex<NodiumApp>>) -> Self {
        debug!("App init");

        NodiumViewConsole { app }
    }

    pub async fn handle_plugin_list(&self) {
        debug!("Handle plugin list");
        debug!("Try to get plugins, lock app");
        let app_locked = self.app.lock().await;
        debug!("App locked, get plugins");
        let plugins = app_locked.plugins.lock().await.get_plugins();

        debug!("Plugins: {:?}", plugins);
        for plugin in plugins {
            println!("- {}", plugin);
        }
    }

    pub async fn handle_reload(&self) {
        debug!("Handle reload");
        debug!("Try to get plugins, lock app");
        let app_locked = self.app.lock().await;
        debug!("App locked, reload plugins");
        app_locked.plugins.lock().await.reload().await;
    }

    pub async fn handle_rebuild(&self) {
        debug!("Handle rebuild");
        debug!("Try to get plugins, lock app");
        let app_locked = self.app.lock().await;
        debug!("App locked, rebuild plugins");
        app_locked.plugins.lock().await.rebuild().await;
    }

}
