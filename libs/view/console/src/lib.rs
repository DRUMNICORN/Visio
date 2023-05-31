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

    async fn handle_rebuild(&self) {
        debug!("Handle rebuild");
        debug!("Try to get plugins, lock app");
        let app_locked = self.app.lock().await;
        debug!("App locked, rebuild plugins");
        app_locked.plugins.lock().await.rebuild().await;
    }
}

use async_trait::async_trait;

#[async_trait]
impl NodiumView for NodiumViewConsole {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // the console view will listen to the console for commands.
        // print in the console a message prompt like shell bash or other console
        // if possible use colors and start with "nodium:  " and wait for user command line input

        // clear console and print welcome message as asci art
        clearscreen::clear().unwrap();
        info!("Welcome to Nodium!\n");

        loop {
            // Print the prompt with color
            stdout()
                .execute(SetForegroundColor(Color::Green))?
                .execute(Print("nodium: "))?
                .execute(ResetColor)?;

            // Read user input
            let mut input = String::new();
            stdin().read_line(&mut input)?;

            // Remove the newline character from the input
            input.pop();

            // check first part " " of input 
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
