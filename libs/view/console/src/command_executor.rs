use crate::command::CommandRegistry;
use crossterm::{
    cursor::MoveToColumn,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;

pub fn execute_command(command: &str, command_registry: &CommandRegistry) {
    let command_parts: Vec<String> = command.split_whitespace().map(String::from).collect();

    match command_parts.get(0) {
        Some(cmd) => {
            match cmd.as_str() {
                "exit" => return,
                "help" => {
                    command_registry.list_commands();
                }
                _ => command_registry.execute(cmd, command_parts[1..].to_vec()), // Check for the entire command
            }
        }
        None => {}
    }

    // Clear the input line
    stdout().execute(MoveToColumn(0)).unwrap();
    stdout().execute(Clear(ClearType::CurrentLine)).unwrap();

    // Print nodium prompt
    if command.trim() == "help" {
        print_nodium_prompt();
    }
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
