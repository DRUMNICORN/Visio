use nodium_app::NodiumApp;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn;

use crate::command::{CommandRegistry, Command};

#[derive(Clone)]
pub struct NodiumConsole {
    app: Arc<Mutex<NodiumApp>>,
    pub command_registry: Arc<CommandRegistry>,
}

impl NodiumConsole {
    pub fn new(app: Arc<Mutex<NodiumApp>>) -> Self {
        let command_registry = Arc::new(CommandRegistry::new());

        // Register commands
        command_registry.register(Command {
            name: String::from("exit"),
            description: String::from("Exit the application"),
            handler: Box::new(|| {
                std::process::exit(0);
            }),
        });

        let app_clone = Arc::clone(&app);
        command_registry.register(Command {
            name: String::from("list"),
            description: String::from("List registered commands"),
            handler: Box::new(move || {
                let app_clone = Arc::clone(&app_clone);
                spawn(async move {
                    handle_plugin_list(&app_clone).await;
                });
            }),
        });

        let app_clone = Arc::clone(&app);
        command_registry.register(Command {
            name: String::from("reload"),
            description: String::from("Reload plugins"),
            handler: Box::new(move || {
                let app_clone = Arc::clone(&app_clone);
                spawn(async move {
                    handle_reload(&app_clone).await;
                });
            }),
        });

        let app_clone = Arc::clone(&app);
        command_registry.register(Command {
            name: String::from("rebuild"),
            description: String::from("Rebuild plugins"),
            handler: Box::new(move || {
                let app_clone = Arc::clone(&app_clone);
                spawn(async move {
                    handle_rebuild(&app_clone).await;
                });
            }),
        });

        // ... Register more commands as needed

        NodiumConsole {
            app,
            command_registry,
        }
    }
}

async fn handle_plugin_list(app: &Arc<Mutex<NodiumApp>>) {
    println!("Handle plugin list");
    println!("Try to get plugins, lock app");
    let app_locked = app.lock().await;
    println!("App locked, get plugins");
    let plugins = app_locked.plugins.lock().await.get_plugins();

    println!("Plugins: {:?}", plugins);
    for plugin in plugins {
        println!("- {}", plugin);
    }
}

async fn handle_reload(app: &Arc<Mutex<NodiumApp>>) {
    println!("Handle reload");
    println!("Try to get plugins, lock app");
    let app_locked = app.lock().await;
    println!("App locked, reload plugins");
    app_locked.plugins.lock().await.reload().await;
}

async fn handle_rebuild(app: &Arc<Mutex<NodiumApp>>) {
    println!("Handle rebuild");
    println!("Try to get plugins, lock app");
    let app_locked = app.lock().await;
    println!("App locked, rebuild plugins");
    app_locked.plugins.lock().await.rebuild().await;
}
