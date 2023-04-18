// nodium/libs/app/src/iced_renderer.rs

use crate::{NodiumApp, Renderer};
use iced::{Application, Settings};

#[derive(Clone)]
pub struct IcedRenderer;

impl Renderer for IcedRenderer {
    fn run(&self, _app: NodiumApp) -> iced::Result {
        crate::NodiumAppIced::run(Settings::default())
    }
}
