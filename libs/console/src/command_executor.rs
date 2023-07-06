use crate::command::CommandRegistry;
use crossterm::{
    cursor::MoveToColumn,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},  
    ExecutableCommand,
};
use tokio::sync::Mutex;
use std::{io::stdout, sync::Arc};

pub async fn execute_command(command: &str, command_registry: Arc<Mutex<CommandRegistry>>) {
    let command_parts: Vec<String> = command.split_whitespace().map(String::from).collect();

    match command_parts.get(0) {
        Some(cmd) => {
            match cmd.as_str() {
                "exit" => return,
                "help" => {
                    command_registry.lock().await.list_commands(0);
                }
                _ => {
                    if let Some(handle) = command_registry.lock().await.execute(cmd, command_parts[1..].to_vec()).await {
                        handle.await.unwrap();
                    }
                }
            }
        }
        None => {}
    }

    // Clear the input line
    stdout().execute(MoveToColumn(0)).unwrap();
    stdout().execute(Clear(ClearType::CurrentLine)).unwrap();

    // Print nodium prompt
    print_nodium_prompt();
}

pub fn print_nodium_prompt() {
    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap()
        .execute(Print("nodium: "))
        .unwrap()
        .execute(ResetColor)
        .unwrap();
}
