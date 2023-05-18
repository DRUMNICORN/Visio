// ignore unused rust
use log::debug;
use nodium_app::{NodiumApp};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use tokio::sync::Mutex;
use std::{
    io::{stdin, stdout},
    sync::Arc,
};

#[derive(Clone)]
pub struct NodiumViewConsole {
    app: Arc<Mutex<NodiumApp>>,
    command_history: Vec<String>,
    current_history_index: Option<usize>,
}

impl NodiumViewConsole {
    pub fn new(app: Arc<Mutex<NodiumApp>>) -> Self {
        debug!("App init");

        NodiumViewConsole {
            app,
            command_history: Vec::new(),
            current_history_index: None,
        }
    }

    pub async fn handle_plugin_list(&self) {
        debug!("Handle plugin list");
        debug!("Try to get plugins");
        let app_lock = self.app.lock().await;
        let plugins_lock = app_lock.plugins.lock().await;
        let plugins = plugins_lock.get_plugins();

        debug!("Plugins: {:?}", plugins);
        for plugin in plugins {
            println!("- {}", plugin.name);
        }
    }

    pub async fn handle_reload(&self) {
      let app_lock = self.app.lock().await;
      let mut plugins_lock = app_lock.plugins.lock().await;
      plugins_lock.reload().await;
    }

    pub async fn handle_version(&self) {
        debug!("Handle version");
        println!("Nodium version: {}", env!("CARGO_PKG_VERSION"));
    }

    pub fn handle_help(&self) {
        debug!("Handle help");
        println!("Nodium help");
        println!("Commands:");
        println!("list - list all plugins");
        println!("version - show nodium version");
        println!("help - show this help");
        println!("reload - reload plugins");
        println!("clear - clear console");
        println!("exit - exit nodium");
    }

    pub fn handle_clear(&self) {
        debug!("Handle clear");
        stdout().execute(Clear(ClearType::All)).unwrap();
    }

    pub fn prompt(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        stdout()
            .execute(SetForegroundColor(Color::Green))?
            .execute(Print("nodium: "))?
            .execute(ResetColor)?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(input.trim().to_owned())
    }

    pub fn handle_up_arrow(&mut self) {
        if let Some(index) = self.current_history_index {
            if index > 0 {
                self.current_history_index = Some(index - 1);
            }
        } else {
            let history_len = self.command_history.len();
            if history_len > 0 {
                self.current_history_index = Some(history_len - 1);
            }
        }
    }

    pub fn handle_down_arrow(&mut self) {
        if let Some(index) = self.current_history_index {
            let history_len = self.command_history.len();
            if index < history_len - 1 {
                self.current_history_index = Some(index + 1);
            } else {
                self.current_history_index = None;
            }
        }
    }

    pub async fn run_loop(&mut self) {
        loop {
            let input = self.prompt().unwrap();
            if !input.is_empty() {
                self.command_history.push(input.clone());
            }
            let arguments: Vec<&str> = input.split_whitespace().collect();
            let command = arguments[0];
            let _arguments = &arguments[1..];
            match command {
              "exit" => break,
              "list" => self.handle_plugin_list().await,
              "version" => self.handle_version().await,
              "help" => self.handle_help(),
              "reload" => self.handle_reload().await,
              "clear" => self.handle_clear(),
              _ => println!("Unknown command: {}", command),
          }
        }
    }
}