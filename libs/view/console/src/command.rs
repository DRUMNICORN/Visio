// command.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::debug;
use std::io::{stdout, Write};
use crossterm::{
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
     queue,
};

use crate::command_executor::{print_nodium_prompt};

pub struct Command {
    pub name: String,
    pub description: String,
    pub handler: Box<dyn Fn() + Send + Sync>,
}

pub struct CommandRegistry {
    commands: Arc<Mutex<HashMap<String, Command>>>,
}

impl CommandRegistry {
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


    pub fn execute(&self, name: &str) {
        if let Some(command) = self.commands.lock().unwrap().get(name) {
            (command.handler)();
        } else {
            queue!(
                stdout(),
                SetForegroundColor(Color::Red), // Set the text color to red
                Print(format!("Unknown command: {}", name)),
                SetForegroundColor(Color::Reset), // Reset the text color
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
