mod iced_app;

use iced_app::NodiumAppIced;

use iced::{Application, Settings};
use nodium_app::{NodiumRenderer, NodiumApp};

#[derive(Clone)]
pub struct IcedRenderer;

impl NodiumRenderer for IcedRenderer {
    fn run(&self, _app: NodiumApp) -> Result<(), Box<dyn std::error::Error>> {
        match crate::NodiumAppIced::run(Settings::default()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub enum Message {
  SwitchToDarkTheme,  
  SwitchToLightTheme,
}


pub struct Theme {
  pub background: iced::Color,
  pub primary: iced::Color,
  pub secondary: iced::Color,
  // Add more theme-specific properties as needed
}

impl Theme {
  pub fn dark() -> Self {
      Self {
          background: iced::Color::from_rgb8(37, 37, 38),
          primary: iced::Color::from_rgb8(40, 44, 52),
          secondary: iced::Color::from_rgb8(60, 64, 72),
      }
  }

  pub fn _light() -> Self {
      Self {
          background: iced::Color::from_rgb8(255, 255, 255),
          primary: iced::Color::from_rgb8(240, 240, 240),
          secondary: iced::Color::from_rgb8(220, 220, 220),
      }
  }
}
