use nodium_pdk::{NodiumWindow, NodiumLayout};
pub trait NodiumView {
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
    fn set_layout(&self, layout: NodiumLayout) -> Result<(), Box<dyn std::error::Error>>;
    fn focus_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>>;
}
