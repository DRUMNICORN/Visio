use crate::command::CommandRegistry;
use std::io::{stdout};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
    cursor::{MoveToColumn},
    terminal::{Clear, ClearType},
};

pub fn execute_command(command: &str, command_registry: &CommandRegistry) {
    match command.trim() {
        "exit" => return,
        "help" => command_registry.clone().list_commands(),
        _ => command_registry.clone().execute(command), // Check for the entire command
    }

    // Clear the input line
    stdout().execute(MoveToColumn(0)).unwrap();
    stdout().execute(Clear(ClearType::CurrentLine)).unwrap();
}

pub fn print_nodium_prompt() {
    stdout()
        .execute(SetForegroundColor(Color::Green)).unwrap()
        .execute(Print("nodium: ")).unwrap()
        .execute(ResetColor).unwrap();
}
