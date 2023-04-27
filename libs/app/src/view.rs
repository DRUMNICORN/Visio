use std::sync::Arc;

use nodium_pdk::{NodiumEvent, NodiumNode, NodiumWindow};
use tokio::sync::Mutex;

use crate::NodiumApp;

pub trait NodiumView: Send + Sync {
    fn run(
        &self,
        // event_callback: Box<dyn Fn(NodiumEvent) + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn add_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>>;
    fn remove_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn update_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    // TODO: Extend the NodiumView trait
}
