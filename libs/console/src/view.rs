use async_trait::async_trait;
use log::info;
use nodium_app::NodiumView;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
};

use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};

use crate::key_handler::handle_key_event;
use crate::command_executor::{execute_command, print_nodium_prompt};
use crate::NodiumConsole;

#[async_trait]
impl NodiumView for NodiumConsole {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Welcome to Nodium!\n"); // Add a line break after the message

        let mut history: Vec<String> = Vec::new();
        let mut history_index: Option<usize> = None;
        let mut current_input = String::new();
        let ctrl_c_pressed = Arc::new(tokio::sync::Mutex::new(false));

        // Set Ctrl+C handler
        let ctrl_c_pressed_clone = Arc::clone(&ctrl_c_pressed);
        tokio::spawn(async move {
            let mut sig_int = signal(SignalKind::interrupt()).expect("Failed to register Ctrl+C handler");
            sig_int.recv().await;
            let mut pressed = ctrl_c_pressed_clone.lock().await;
            *pressed = true;
        });

        
        print_nodium_prompt();
        loop {

            let mut input = String::new();
            let mut ctrl_c_pressed = ctrl_c_pressed.lock().await;
            if input.trim() == "exit" || *ctrl_c_pressed {
                break;
            }
            loop {
                if event::poll(std::time::Duration::from_millis(100))? {
                    if let CEvent::Key(key_event) = event::read()? {
                        handle_key_event(
                            key_event,
                            &mut input,
                            &history,
                            &mut history_index,
                            &mut current_input,
                            &mut *ctrl_c_pressed,
                        );
                        
                        if key_event.code == KeyCode::Enter {
                            break;
                        }
                    }
                }
            }
            history.push(input.clone());

            execute_command(&input, self.command_registry).await;

            if input.trim() == "exit" {
                break;
            }
        }

        Ok(())
    }
}
