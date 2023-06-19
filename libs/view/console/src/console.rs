use std::sync::Arc;
use tokio::task::spawn;
use tokio::sync::Mutex;
use crate::command::{CommandRegistry, Command};
use crate::app_handler::{handle_flow_add, handle_flow_remove, handle_flow_list, handle_plugins_list, handle_plugins_rebuild, handle_plugins_reload};
use nodium_app::NodiumApp;
use log::{info, warn};

pub struct NodiumConsole {
    pub app: Arc<Mutex<NodiumApp>>,
    pub command_registry: Arc<CommandRegistry>,
}

impl NodiumConsole {
    pub fn new(app: Arc<Mutex<NodiumApp>>) -> Self {
        // Initialize the logging backend (env_logger in this case)
        let command_registry = Arc::new(CommandRegistry::new(vec![
            Command {
                name: String::from("exit"),
                aliases: vec!["^C".to_string()].into_iter().collect(),
                description: String::from("Exit the application"),
                handler: Box::new(|_| {
                    std::process::exit(0);
                }),
                sub_commands: None,
            },
            Command {
                name: String::from("flow"),
                aliases: vec!["f".to_string()].into_iter().collect(),
                description: String::from("Manage flows"),
                handler: Box::new(|_| {
                    spawn(async {()})
                }),
                sub_commands: Some(Arc::new(CommandRegistry::new(vec![
                    Command {
                        name: String::from("list"),
                        aliases: vec!["l".to_string()].into_iter().collect(),
                        description: String::from("List available flows"),
                        handler: {
                            let app_clone = Arc::clone(&app);
                            Box::new(move |_args| {
                                let app_clone_inner = Arc::clone(&app_clone);
                                spawn(async move {
                                    handle_flow_list(&app_clone_inner).await
                                })
                            })
                        },
                        sub_commands: None,
                    },
                    
                    Command {
                        name: String::from("add"),
                        aliases: vec!["a".to_string()].into_iter().collect(),
                        description: String::from("Create flows"),
                        handler: {
                            let app_clone = Arc::clone(&app);
                            Box::new(move |args| {
                                let app_clone = Arc::clone(&app_clone);
                                let args = args.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
                                spawn(async move {
                                    handle_flow_add(&app_clone, args).await
                                })
                            })
                        },
                        sub_commands: None,
                    },
                    Command {
                        name: String::from("remove"),
                        aliases: vec!["r".to_string()].into_iter().collect(),
                        description: String::from("Remove flows"),
                        handler: {
                            let app_clone = Arc::clone(&app);
                            Box::new(move |args| {
                                if let Some(flow_name) = args.get(0) {
                                    let app_clone = Arc::clone(&app_clone);
                                    let flow_name = flow_name.to_string();
                                    spawn(async move {
                                        handle_flow_remove(&app_clone, flow_name).await
                                    })
                                    
                                } else {
                                    println!("Error: You must provide a name for the flow.");
                                    spawn(async {()})
                                }
                            })
                        },
                        sub_commands: None,
                    },                      
                ]))),
            },
            Command {
                name: String::from("plugins"),
                aliases: vec!["p".to_string()].into_iter().collect(),
                description: String::from("Manage plugins"),
                handler: Box::new(|_| {
                    spawn(async {()})
                }),
                sub_commands: Some(Arc::new(CommandRegistry::new(vec![
                    Command {
                        name: String::from("list"),
                        aliases: vec!["l".to_string()].into_iter().collect(),
                        description: String::from("List registered commands"),
                        handler: {
                            let app_clone = Arc::clone(&app);
                            Box::new(move |_| {
                                let app_clone_inner = Arc::clone(&app_clone);
                                spawn(async move {
                                    handle_plugins_list(&app_clone_inner).await
                                })
                            })
                        },
                        sub_commands: None,
                    },
                    Command {
                        name: String::from("reload"),
                        aliases: vec!["rl".to_string()].into_iter().collect(),
                        description: String::from("Reload plugins"),
                        handler: {
                            let app_clone = Arc::clone(&app);
                            Box::new(move |_| {
                                let app_clone_inner = Arc::clone(&app_clone);
                                spawn(async move {
                                    handle_plugins_reload(&app_clone_inner).await
                                })
                            })
                        },
                        sub_commands: None,
                    },
                    Command {
                        name: String::from("rebuild"),
                        aliases: vec!["rb".to_string()].into_iter().collect(),
                        description: String::from("Rebuild plugins"),
                        handler: {
                            let app_clone = Arc::clone(&app);
                            Box::new(move |_| {
                                let app_clone_inner = Arc::clone(&app_clone);
                                spawn(async move {
                                    handle_plugins_rebuild(&app_clone_inner).await
                                })
                            })
                        },
                        sub_commands: None,
                    },
                ]))),
            },
        ]));

        Self {
            app,
            command_registry,
        }
    }
    pub fn execute_command(&self, command: &str, args: Vec<String>) {
        if let Some(join_handle) = self.command_registry.execute(command, args.clone()) {
            // Spawn the command handler task and await its completion
            spawn(async move {
                join_handle.await.expect("Command execution failed");
            });
        } else {
            warn!("Unknown command: {}", command);
        }
    }

    pub fn print_help(&self) {
        info!("Available commands:");
        self.command_registry.list_commands(0);
    }
}