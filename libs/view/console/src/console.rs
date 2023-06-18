// console.rs
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn;

use crate::command_executor::print_nodium_prompt;
use crate::command::{CommandRegistry, Command};
use nodium_app::NodiumApp;

pub struct NodiumConsole {
    pub app: Arc<Mutex<NodiumApp>>,
    pub command_registry: Arc<CommandRegistry>,
    pub commadns : Vec<String>,
}

impl Clone for NodiumConsole {
    fn clone(&self) -> Self {
        Self {
            app: Arc::clone(&self.app),
            command_registry: Arc::clone(&self.command_registry),
            commands: self.commands.clone(),
        }
    }
}

use std::collections::HashMap;

pub struct NodiumConsole {
    pub app: Arc<Mutex<NodiumApp>>,
    pub command_registry: Arc<CommandRegistry>,
    pub commands: HashMap<String, Command>,
}

impl NodiumConsole {
    pub fn new(app: Arc<Mutex<NodiumApp>>) -> Self {
        let command_registry = Arc::new(CommandRegistry::new());

        // Register commands
        let mut commands = HashMap::new();

        commands.insert(
            String::from("exit"),
            Command {
                name: String::from("exit"),
                description: String::from("Exit the application"),
                handler: Box::new(|_| {
                    std::process::exit(0);
                }),
                sub_commands: None,
            },
        );

        let app_clone = Arc::clone(&app);
        commands.insert(
            String::from("list"),
            Command {
                name: String::from("list"),
                description: String::from("List registered commands"),
                handler: Box::new(move |_| {
                    let app_clone = Arc::clone(&app_clone);
                    spawn(async move {
                        handle_plugin_list(&app_clone).await;
                    });
                }),
                sub_commands: None,
            },
        );

        let app_clone = Arc::clone(&app);
        commands.insert(
            String::from("reload"),
            Command {
                name: String::from("reload"),
                description: String::from("Reload plugins"),
                handler: Box::new(move |_| {
                    let app_clone = Arc::clone(&app_clone);
                    spawn(async move {
                        handle_reload(&app_clone).await;
                    });
                }),
                sub_commands: None,
            },
        );

        let app_clone = Arc::clone(&app);
        commands.insert(
            String::from("rebuild"),
            Command {
                name: String::from("rebuild"),
                description: String::from("Rebuild plugins"),
                handler: Box::new(move |_| {
                    let app_clone = Arc::clone(&app_clone);
                    spawn(async move {
                        handle_rebuild(&app_clone).await;
                    });
                }),
                sub_commands: None,
            },
        );
        let mut flow_sub_commands = HashMap::new();

        // Register the "flow" command with sub-commands
        commands.insert(
            String::from("flow"),
            Command {
                name: String::from("flow"),
                description: String::from("`flow`: Flow commands `flow <cmd>`: to execute a command"),
                handler: Box::new(move |args| {
                    let app_clone = Arc::clone(&app_clone);
                    spawn(async move {
                        handle_flow(&app_clone, args).await;
                    });
                    print_nodium_prompt();
                }),
                sub_commands: [
                    Command {
                        name: String::from("list"),
                        description: String::from("List available flows"),
                        handler: Box::new(|args| {
                            // Handler logic for the "flow list" command
                            println!("Handling 'flow list' command with args: {:?}", args);
                        }),
                        sub_commands: None,
                    },
                    Command {
                        name: String::from("execute"),
                        description: String::from("Execute a specific flow"),
                        handler: Box::new(|args| {
                            // Handler logic for the "flow execute" command
                            println!("Handling 'flow execute' command with args: {:?}", args);
                        }),
                        sub_commands: None,
                    }
                ],
            },
        );

        command_registry.commands.lock().unwrap().extend(commands);

        NodiumConsole {
            app,
            command_registry,
            commands: command_registry.commands,
        }
    }
}


async fn handle_plugin_list(app: &Arc<Mutex<NodiumApp>>) {
    let app_locked = app.lock().await;
    let plugins = app_locked.plugins.lock().await.get_plugins();

    println!("Plugins: {:?}", plugins);
    for plugin in plugins {
        println!("- {}", plugin);
    }

    print_nodium_prompt();
}

async fn handle_reload(app: &Arc<Mutex<NodiumApp>>) {
    let app_locked = app.lock().await;
    app_locked.plugins.lock().await.reload().await;
    print_nodium_prompt();
}

async fn handle_rebuild(app: &Arc<Mutex<NodiumApp>>) {
    let app_locked = app.lock().await;
    app_locked.plugins.lock().await.rebuild().await;
    print_nodium_prompt();
}

async fn handle_flow(app: &Arc<Mutex<NodiumApp>>, args: Vec<String>) {
    if let Some(cmd) = args.get(0) {
        match cmd.as_str() {
            "list" => handle_flow_list(app).await,
            // Add more flow commands here
            _ => println!("Unknown flow command: {}", cmd),
        }
    } else {
        println!("Usage: flow <cmd>");
    }
}

async fn handle_flow_list(app: &Arc<Mutex<NodiumApp>>) {
    println!("Handle flow list");
    let app_locked = app.lock().await;
    let flows = app_locked.flows.lock().await.get_flows().await;
    for flow in flows {
        println!("- {}", flow.0);
    }

    print_nodium_prompt();
}
