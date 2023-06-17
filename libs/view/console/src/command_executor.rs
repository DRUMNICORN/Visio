use crate::command::CommandRegistry;
use crossterm::{
    cursor::MoveToColumn,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;

pub fn execute_command(command: &str, command_registry: &CommandRegistry) {
    match command.trim() {
        "exit" => return,
        "help" => {
            command_registry.clone().list_commands();
        }
        _ => command_registry.clone().execute(command), // Check for the entire command
    }

    // Clear the input line
    stdout().execute(MoveToColumn(0)).unwrap();
    stdout().execute(Clear(ClearType::CurrentLine)).unwrap();
    
    // print_nodium_prompt(); only when command is not help or exit
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
