use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::debug;
use std::io::{stdout, Write};
use crossterm::{
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    queue,
};

use crate::command_executor::print_nodium_prompt;

pub struct Command {
    pub name: String,
    pub description: String,
    pub handler: Box<dyn Fn(Vec<String>) + Send + Sync>,
    pub sub_commands: Option<Arc<CommandRegistry>>,
}

pub struct CommandRegistry {
    pub commands: Arc<Mutex<HashMap<String, Command>>>,
}

impl CommandRegistry {
    pub fn fromCommandArgs(commands: Vec<Command>) -> CommandRegistry {
        let commands: HashMap<String, Command> = HashMap::new();
        for command in commands {
            commands.insert(command.0.clone(), command.1);
        }
        let commands = Arc::new(Mutex::new(commands));
        CommandRegistry { commands }
    }


    pub fn new() -> Self {
        CommandRegistry {
            commands: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, command: Command) {
        debug!("Registering command: {}", command.name);
        self.commands
            .lock()
            .unwrap()
            .insert(command.name.clone(), command);
    }

    pub fn execute(&self, name: &str, args: Vec<String>) {
        if let Some(command) = self.commands.lock().unwrap().get(name) {
            if let Some(sub_commands) = &command.sub_commands {
                if args.is_empty() {
                    sub_commands.list_commands();
                } else if let Some(sub_command_name) = args.get(0) {
                    sub_commands.execute(sub_command_name, args[1..].to_vec());
                }
            } else {
                (command.handler)(args);
            }
        } else {
            queue!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("Unknown command: {}", name)),
                SetForegroundColor(Color::Reset),
                Print("\n"),
                Clear(ClearType::UntilNewLine),
            )
            .unwrap();
            stdout().flush().unwrap();
        }
    }

    pub fn list_commands(&self) {
        let commands = self.commands.lock().unwrap();
        println!("Registered commands:");
        for command in commands.values() {
            println!("- {}: {}", command.name, command.description);
        }

        print_nodium_prompt();
    }
}
