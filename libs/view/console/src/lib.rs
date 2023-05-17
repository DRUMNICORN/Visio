// ignore unused rust
use log::{debug, info};
use nodium_app::{NodiumApp, NodiumView};
// use nodium_events::NodiumEventBus;
use nodium_pdk::{LayoutType, NodiumLayout, NodiumWindow};
// use serde_json::{from_str, to_value};
// use std::sync::{mpsc, Arc};
// use tokio::sync::mpsc::Sender;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::{
    io::{stdin, stdout, Write},
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct NodiumViewConsole {
    // event_sender: Arc<Mutex<Sender<Option<Event<'static, ()>>>>>,
    // event_loop: Arc<Mutex<EventLoop<()>>>,
    // platform: Arc<Mutex<Platform>>,
    // ctx: Arc<egui::CtxRef>,
    // sender: Sender<Event<'static, ()>>,
    app: Arc<Mutex<NodiumApp>>,
}

impl NodiumViewConsole {
    pub fn new(app: Arc<Mutex<NodiumApp>>) -> Self {
        debug!("App init");

        NodiumViewConsole { app }
    }

    async fn handle_plugin_list(&self) {
        debug!("Handle plugin list");
        debug!("Try to get plugins, lock app");
        let app_locked = self.app.lock().await;
        debug!("App locked, get plugins");
        let plugins = app_locked.plugins.lock().await.get_plugins();

        debug!("Plugins: {:?}", plugins);
        for plugin in plugins {
            println!("- {}", plugin);
        }
    }

    async fn handle_reload(&self) {
        debug!("Handle reload");
        debug!("Try to get plugins, lock app");
        let app_locked = self.app.lock().await;
        debug!("App locked, reload plugins");
        app_locked.plugins.lock().await.reload().await;
    }

    async fn handle_version(&self) {
        debug!("Handle version");
        println!("Nodium version: {}", env!("CARGO_PKG_VERSION"));
    }

    fn handle_help(&self) {
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

    fn handle_clear(&self) {
        debug!("Handle clear");
        match clearscreen::clear() {
            Ok(_) => {}
            Err(e) => {
                println!("Error clearing screen: {}", e);
            }
        }
    }

    fn prompt(&self) -> Result<String, Box<dyn std::error::Error>> {
      stdout()
          .execute(SetForegroundColor(Color::Green))?
          .execute(Print("nodium: "))?
          .execute(ResetColor)?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(input)
    }

    async fn run_loop(&self) {
        loop {
            let input = self.prompt().unwrap();
            let arguments: Vec<&str> = input.split_whitespace().collect();
            let command = arguments[0];
            let arguments = &arguments[1..];
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

impl NodiumView for NodiumViewConsole {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // the console view will listen to the console for commands.
        // print in the console a message prompt like shell bash or other console
        // if possible use colors and start with "nodium:  " and wait for user command line input

        // clear console and print welcome message as asci art
        self.handle_clear();

        // start the console loop as async task
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.run_loop());

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
        let layout_type = layout.layout_type;
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
