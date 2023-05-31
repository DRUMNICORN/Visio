// nodium_view_console_impl.rs
use async_trait::async_trait;
// nodium_view_console.rs
use log::{debug, info};
use nodium_app::{ NodiumView};
use nodium_pdk::{ NodiumLayout, NodiumWindow};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
    cursor::{MoveLeft, MoveToColumn, SavePosition, RestorePosition},
    terminal::{Clear, ClearType},
    event::{self, Event as CEvent, KeyCode}
};

use std::{
    io::{stdout},
};

use crate::NodiumViewConsole;

#[async_trait]
impl NodiumView for NodiumViewConsole {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // the console view will listen to the console for commands.
        // print in the console a message prompt like shell bash or other console
        // if possible use colors and start with "nodium:  " and wait for user command line input

        // clear console and print welcome message as asci art
        clearscreen::clear().unwrap();
        info!("Welcome to Nodium!\n");

        let mut history: Vec<String> = Vec::new();
        let mut history_index: usize = 0;

        loop {
            // Print the prompt with color
            stdout()
                .execute(SetForegroundColor(Color::Green))?
                .execute(Print("nodium: "))?
                .execute(ResetColor)?;

            // Read user input using crossterm events
            let mut input = String::new();
            loop {
                stdout().execute(SavePosition)?;
                if let CEvent::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Enter => {
                            println!();
                            break;
                        }
                        KeyCode::Backspace => {
                            input.pop();
                            stdout()
                                .execute(MoveLeft(1))?
                                .execute(Clear(ClearType::UntilNewLine))?;
                        }
                        KeyCode::Up => {
                            if history_index > 0 {
                                history_index -= 1;
                                input = history[history_index].clone();
                                stdout().execute(MoveToColumn(9))?;
                                stdout().execute(Clear(ClearType::UntilNewLine))?;
                                print!("{}", input);
                            }
                        }
                        KeyCode::Down => {
                            if history_index < history.len() - 1 {
                                history_index += 1;
                                input = history[history_index].clone();
                                stdout().execute(MoveToColumn(9))?;
                                stdout().execute(Clear(ClearType::UntilNewLine))?;
                                print!("{}", input);
                            }
                        }
                        KeyCode::Char(c) => {
                            input.push(c);
                            print!("{}", c);
                        }
                        _ => {}
                    }
                }
                stdout().execute(RestorePosition)?;
            }

            // Add the input to the history
            history.push(input.clone());
            history_index = history.len();

            let init = input.split(" ").collect::<Vec<&str>>()[0];
            match init {
                "exit" => break,
                "list" => self.handle_plugin_list().await,
                "version" => println!("Nodium version: {}", env!("CARGO_PKG_VERSION")),
                "help" => println!("Help"),
                "reload" => self.handle_reload().await,
                "clear" => clearscreen::clear().unwrap(),
                "rebuild" => self.handle_rebuild().await,
                _ => println!("You entered: {}", input),
            }
        }

        Ok(())
    }

    fn add_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>> {
        let window_name = window.name();
        debug!("Add window: {}", window_name);
        Ok(())
    }

    fn remove_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let window_name = window.name();
        debug!("Remove window: {}", window_name);
        Ok(())
    }

    fn update_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let window_name = window.name();
        debug!("Update window: {}", window_name);
        Ok(())
    }
    fn set_layout(&self, layout: NodiumLayout) -> Result<(), Box<dyn std::error::Error>> {
        let _layout_type = layout.layout_type;
        debug!("Set layout...");
        Ok(())
    }

    fn focus_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let window_name = window.name();
        debug!("Focus window: {}", window_name);
        Ok(())
    }
}
