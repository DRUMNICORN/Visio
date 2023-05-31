use crate::{NodiumUiComponent, NodiumWindow};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub enum LayoutType {
    Vertical(Vec<Arc<Mutex<dyn NodiumWindow>>>),
    Horizontal(Vec<Arc<Mutex<dyn NodiumWindow>>>),
    Grid(Vec<Vec<Arc<Mutex<dyn NodiumWindow>>>>),
}

pub struct NodiumLayout {
    pub layout_type: LayoutType,
}

impl NodiumLayout {
    pub fn new_vertical() -> Self {
        NodiumLayout {
            layout_type: LayoutType::Vertical(Vec::new()),
        }
    }

    pub fn new_horizontal() -> Self {
        NodiumLayout {
            layout_type: LayoutType::Horizontal(Vec::new()),
        }
    }

    pub fn new_grid(rows: usize, cols: usize) -> Self {
        NodiumLayout {
            layout_type: LayoutType::Grid(vec![
                vec![Arc::new(Mutex::new(EmptyWindow)); cols];
                rows
            ]),
        }
    }

    pub fn add_window(&self, _window: Arc<Mutex<dyn NodiumWindow>>) {
        match &self.layout_type {
            LayoutType::Vertical(_window_list) => {
                // window_list.push(window);
            }
            LayoutType::Horizontal(_window_list) => {
                // window_list.push(window);
            }
            LayoutType::Grid(_) => {
                // Implement grid window adding logic
            }
        }
    }

    pub fn remove_window(&self, _window: Arc<Mutex<dyn NodiumWindow>>) {
        match &self.layout_type {
            LayoutType::Vertical(_window_list) => {
                // window_list.retain(|w| !Arc::ptr_eq(w, &window));
            }
            LayoutType::Horizontal(_window_list) => {
                // window_list.retain(|w| !Arc::ptr_eq(w, &window));
            }
            LayoutType::Grid(_grid) => {
                // Implement grid window removing logic
            }
        }
    }
}

struct EmptyWindow;

impl NodiumWindow for EmptyWindow {
    fn name(&self) -> String {
        String::new()
    }
    fn icon(&self) -> String {
        String::new()
    }
    fn title(&self) -> String {
        String::new()
    }
    fn content(&self) -> NodiumUiComponent {
        NodiumUiComponent::Text(String::new())
    }
}
