use std::io::{stdout, Write};
use crossterm::{
    style::Print,
    cursor::{MoveLeft, MoveToColumn},
    terminal::{Clear, ClearType},
    event::{KeyCode, KeyEvent},
    execute, queue,
};
use crossterm::event::KeyModifiers;

pub fn handle_key_event(
    key_event: KeyEvent,
    input: &mut String,
    history: &[String],
    history_index: &mut Option<usize>,
    current_input: &mut String,
    ctrl_c_pressed: &mut bool,
) {
    match key_event.code {
        KeyCode::Enter => {
            *input = input.trim().to_string(); // Trim the input to remove leading/trailing whitespace
        }
        KeyCode::Backspace => {
            if !input.is_empty() {
                input.pop();
                execute!(stdout(), MoveLeft(1), Print(" "), MoveLeft(1)).unwrap();
            }
        }
        KeyCode::Up => {
            if history.is_empty() {
                // Do nothing if history is empty
            } else {
                if history_index.is_none() {
                    *current_input = input.clone();
                }
                let new_index = match history_index {
                    Some(0) => Some(0),
                    Some(index) => Some(*index - 1),
                    None => Some(history.len() - 1),
                };
                if let Some(index) = new_index {
                    *input = history[index].clone();
                    remove_escape_sequences(input); // Remove escape sequences
                    queue!(stdout(), MoveToColumn(0), Clear(ClearType::UntilNewLine)).unwrap();
                    print!("{}", input);
                    execute!(stdout(), MoveToColumn(input.len() as u16)).unwrap(); // Update cursor position
                }
                *history_index = new_index;
            }
        }
        KeyCode::Down => {
            let new_index = match history_index {
                Some(index) if *index < history.len() - 1 => Some(*index + 1),
                _ => None,
            };
            if let Some(index) = new_index {
                *input = history[index].clone();
                remove_escape_sequences(input); // Remove escape sequences
                queue!(stdout(), MoveToColumn(0), Clear(ClearType::UntilNewLine)).unwrap();
                print!("{}", input);
                execute!(stdout(), MoveToColumn(input.len() as u16)).unwrap(); // Update cursor position
            } else {
                *input = current_input.clone();
                queue!(stdout(), MoveToColumn(0), Clear(ClearType::UntilNewLine)).unwrap();
                print!("{}", input);
                execute!(stdout(), MoveToColumn(input.len() as u16)).unwrap(); // Update cursor position
            }
            *history_index = new_index;
        }
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            *ctrl_c_pressed = true;
        }
        KeyCode::Char(c) => {
            input.push(c);
            //print!("{}", c); // Print the character
            stdout().flush().unwrap(); // Flush the output to make sure it is displayed immediately
        }
        KeyCode::Left | KeyCode::Right | KeyCode::Esc => {} // Ignore Left, Right, and Esc keys
        _ => {
            // Ignore other keys
        }
    }
}

fn remove_escape_sequences(input: &mut String) {
    input.retain(|c| !c.is_control());
}
