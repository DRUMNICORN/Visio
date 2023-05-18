mod view;
pub use crate::view::NodiumViewConsole;

use log::debug;
use nodium_app::{NodiumView};
use nodium_pdk::{NodiumWindow, NodiumLayout};

impl NodiumView for NodiumViewConsole {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // the console view will listen to the console for commands.
        // print in the console a message prompt like shell bash or other console
        // if possible use colors and start with "nodium:  " and wait for user command line input

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
