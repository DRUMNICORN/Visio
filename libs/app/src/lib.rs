// libs/nodium_ui/src/lib.rs TODO: https://docs.github.com/en/get-started/using-git/splitting-a-subfolder-out-into-a-new-repository
// path: libs/app/src/renderer.rs

// pub trait Renderer {
//   fn run(&self, app: crate::NodiumApp);
//   fn clone_box(&self) -> Box<dyn Renderer>;
// }

mod iced_renderer;
pub use iced_renderer::IcedRenderer;


pub trait Renderer: RendererClone {
  fn run(&self, app: NodiumApp) -> iced::Result;
}

pub trait RendererClone {
  fn clone_box(&self) -> Box<dyn Renderer>;
}

impl<T> RendererClone for T
where
  T: 'static + Renderer + Clone,
{
  fn clone_box(&self) -> Box<dyn Renderer> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn Renderer> {
  fn clone(&self) -> Box<dyn Renderer> {
    self.clone_box()
  }
}

use crates_io_api::{Crate, SyncClient};

use log::debug;
use regex::Regex;

use nodium_events::EventBus;
// use serde_json::json;

use std::hash::{Hash, Hasher};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

#[cfg(feature = "egui-renderer")]
use eframe::{self, epi};

#[cfg(feature = "tauri-renderer")]
use tauri::{self, Builder};

// constants for extension crates
const EXTENSION_CRATE_PREFIX: &str = "nodium_"; // prefix for extension crates on crates.io are "nodium_"

pub struct WrappedCrate(pub Crate);

impl PartialEq for WrappedCrate {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Eq for WrappedCrate {}

impl Hash for WrappedCrate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

impl Clone for WrappedCrate {
    fn clone(&self) -> Self {
        WrappedCrate(self.0.clone())
    }
}

#[cfg(feature = "egui-renderer")]
use nodium_app_egui::EguiRenderer;

#[cfg(feature = "tauri-renderer")]
use nodium_app_tauri::TauriRenderer;


pub struct NodiumApp {
    search_query: String,
    crates: Arc<Mutex<HashSet<WrappedCrate>>>,
    event_bus: Arc<EventBus>,
    renderer: Box<dyn Renderer>,
    fetching: Arc<Mutex<bool>>,
}

impl Clone for NodiumApp {
  fn clone(&self) -> Self {
      NodiumApp {
          event_bus: self.event_bus.clone(),
          renderer: self.renderer.clone_box(),
          crates: self.crates.clone(),
          search_query: self.search_query.clone(),
          fetching: self.fetching.clone(),
      }
  }
}


impl NodiumApp {
    pub fn new(event_bus: Arc<EventBus>, renderer: Box<dyn Renderer>) -> Self {
        let crates = Arc::new(Mutex::new(HashSet::new()));
        let fetching = Arc::new(Mutex::new(true));
        let event_bus = event_bus.clone();
        let crates_clone = crates.clone();
        let fetching_clone = fetching.clone();
        tokio::spawn(async move {
            Self::fetch_nodium_extension_crates(crates_clone, fetching_clone).await;
        });
        NodiumApp {
            search_query: String::new(),
            crates,
            event_bus,
            renderer: renderer,
            fetching,
        }
    }

    pub fn run(&self) {
      self.renderer.run(self.clone());
  }

    async fn fetch_nodium_extension_crates(
        crates: Arc<Mutex<HashSet<WrappedCrate>>>,
        fetching: Arc<Mutex<bool>>,
    ) {
        // Create a crates.io client
        let client = SyncClient::new();
        let pattern = Regex::new(&format!("^{}.*", EXTENSION_CRATE_PREFIX)).unwrap();

        // Fetch the list of crates from crates.io
        let query = Some(EXTENSION_CRATE_PREFIX.to_string());
        let result = client.all_crates(query);

        // Filter the list of crates to only include nodium extension crates
        let found_crates = match result {
            Ok(crates) => crates,
            Err(e) => {
                debug!("Error fetching crates: {}", e);
                let mut fetching = fetching.lock().unwrap();
                *fetching = false;
                return;
            }
        };

        // Update the list of crates
        let mut crates_guard = crates.lock().unwrap();
        for krate in found_crates {
            if pattern.is_match(&krate.name) {
                debug!("Found nodium extension crate: {}", krate.name);
                crates_guard.insert(WrappedCrate(krate));
            }
        }

        let mut fetching = fetching.lock().unwrap();
        *fetching = false;
    }

    // fn btn_install(&self, krate: Crate) -> impl FnOnce(&mut Ui) {
    //     let event_bus = self.event_bus.clone();
    //     move |ui: &mut Ui| {
    //         ui.label(krate.name.replace(EXTENSION_CRATE_PREFIX, ""));
    //         if ui.button("Install").clicked() {
    //             let payload = json!({
    //               "crate_name": krate.clone().name,
    //               "crate_version": krate.clone().max_version
    //             })
    //             .to_string();
    //             debug!("Installing crate: {}", payload);
    //             // warp in tokio task
    //             tokio::spawn(async move {
    //                 event_bus.emit("CrateInstall", payload).await;
    //             });
    //         }
    //     }
    // }
}
// use tokio::runtime::Handle;
// impl epi::App for NodiumApp {
//     fn name(&self) -> &str {
//         "Nodium"
//     }

//     fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
//         let mut style: egui::Style = (*ctx.style()).clone();
//         // it shoud look clean like vs code
//         style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(0x2b, 0x2b, 0x2b);
//         style.visuals.widgets.noninteractive.corner_radius = 0.0;
//         style.visuals.widgets.noninteractive.expansion = 0.0;
//         style.visuals.dark_mode = true;
//         style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(64, 64, 64); // Set custom widget background color
//         ctx.set_style(style);


//         // Create a modern app layout
//         egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
//             ui.horizontal(|ui| {
//                 ui.label("App Title");
//                 ui.separator();
//                 ui.label("File");
//                 ui.label("Edit");
//                 ui.label("View");
//                 // Add more menu items
//             });
//         });

//         egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
//             ui.label("Status bar content");
//         });

//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.label("Main content area");
//             // Add your main content here
//         });

//         egui::SidePanel::left("left_panel").show(ctx, |ui| {
//             ui.label("Left sidebar content");
//             // Add your left sidebar content here
//         });

//         egui::SidePanel::right("right_panel").show(ctx, |ui| {
//             ui.label("Right sidebar content");
//             // Add your right sidebar content here
//         });
//     }
// }
// nodium/libs/app/src/lib.rs

use iced::{Application, Command, Element, Settings, Text};

pub struct NodiumAppIced {
    // add any state variables you may need
}

impl Application for NodiumAppIced {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = ();

    fn new(_flags: ()) -> (NodiumAppIced, Command<Self::Message>) {
        (NodiumAppIced {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Nodium")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Hello, Iced!").into()
    }
}


