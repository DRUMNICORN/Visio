use std::sync::Arc;

use nodium_pdk::{NodiuimNode, NodiuimWindow, NodiumEvent};
use tokio::sync::Mutex;

use crate::NodiumApp;

pub trait NodiumView: NodiumRendererClone + Send + Sync {
    // app management
    fn run(&self, app: Arc<Mutex<NodiumApp>>) -> Result<(), Box<dyn std::error::Error>>;
    // fn clone_box(&self) -> Box<dyn NodiumRenderer>;

    // ui management
    fn add_window(&self, window: NodiuimWindow) -> Result<(), Box<dyn std::error::Error>>;
    fn remove_window(&self, window: NodiuimWindow) -> Result<(), Box<dyn std::error::Error>>;

    fn update_window(&self, window: NodiuimWindow) -> Result<(), Box<dyn std::error::Error>>;

    fn add_node(&self, node: NodiuimNode) -> Result<(), Box<dyn std::error::Error>>;
    fn remove_node(&self, node: NodiuimNode) -> Result<(), Box<dyn std::error::Error>>;

    // event management

    fn add_event(&self, event: NodiumEvent) -> Result<(), Box<dyn std::error::Error>>;
    fn remove_event(&self, event: NodiumEvent) -> Result<(), Box<dyn std::error::Error>>;
    fn update_event(&self, event: NodiumEvent) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait NodiumRendererClone {
    fn clone_box(&self) -> Box<dyn NodiumView>;
}

impl<T> NodiumRendererClone for T
where
    T: 'static + NodiumView + Clone,
{
    fn clone_box(&self) -> Box<dyn NodiumView> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn NodiumView> {
    fn clone(&self) -> Box<dyn NodiumView> {
        self.clone_box()
    }
}
