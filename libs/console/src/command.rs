use crossterm::{
    queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use log::debug;
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::sync::Arc;

// use HashMap
use std::collections::HashMap;

use tokio::{task::JoinHandle, sync::Mutex};
pub type CommandHandler = Box<dyn Fn(Vec<&str>) -> JoinHandle<()> + Send + Sync>;
pub struct Command {
    pub name: String,
    pub description: String,
    pub handler: CommandHandler,
    pub sub_commands: Option<Arc<CommandRegistry>>,
    pub aliases: HashSet<String>,
}

pub struct CommandRegistry {
    pub commands: Arc<Mutex<HashMap<String, Command>>>,
}

impl CommandRegistry {
    pub fn new(commands: Vec<Command>) -> Self {
        let command_map = commands
            .into_iter()
            .map(|command| (command.name.clone(), command))
            .collect();

        Self {
            commands: Arc::new(Mutex::new(command_map)),
        }
    }

    pub async fn register(&self, command: Command) {
        debug!("Registering command: {}", command.name);
        self.commands
            .lock()
            .await
            .insert(command.name.clone(), command);
    }

    pub async fn execute(&self, name: &str, args: Vec<String>) -> Option<JoinHandle<()>> {
        let binding = self.commands.lock().await;
        if let Some((_, command)) = binding
            .iter()
            .find(|(_, command)| command.name == name || command.aliases.contains(name))
        {
            if let Some(sub_commands) = &command.sub_commands {
                if args.is_empty() {
                    sub_commands.list_commands(0);
                } else if let Some(sub_command_name) = args.get(0) {
                    sub_commands.execute(sub_command_name, args[1..].to_vec());
                }
            } else {
                let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                return Some((command.handler)(args_str));
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
        None
    }
    pub async fn list_commands(&self, depth: usize) {
        let commands = self.commands.lock().await;
        if depth == 0 {
            println!("Commands:");
        }
    
        for command in commands.values() {
            let tabs = "    ".repeat(depth);
            let prefix = format!("{}- ", tabs);
    
            // Display aliases in parentheses after the command name
            let aliases = if !command.aliases.is_empty() {
                format!(" ({})", command.aliases.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))
            } else {
                String::new()
            };
    
            queue!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(&prefix),
                SetForegroundColor(Color::Green),
                Print(&command.name),
                SetForegroundColor(Color::Yellow),
                Print(&aliases),
                SetForegroundColor(Color::White),
                Print(": "),
                SetForegroundColor(Color::Reset),
                Print(&command.description),
                Print("\n"),
            )
            .unwrap();
    
            if let Some(sub_commands) = &command.sub_commands {
                sub_commands.list_commands(depth + 1);
            }
        }
    
        // Flush the output to ensure all queued operations are executed.
        stdout().flush().unwrap();
    }
    
}
