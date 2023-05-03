use std::sync::{mpsc, Arc};
use tokio::sync::Mutex;

use egui::{CtxRef, Rgba};
use egui_winit_platform::{Platform, PlatformDescriptor};
use nodium_app::NodiumView;
use nodium_events::NodiumEventBus;
use nodium_pdk::{NodiumEvent, NodiumNode, NodiumWindow};
use serde_json::{from_str, to_value};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// EGUI view
#[derive(Clone)]
pub struct NodiumViewEgui {
    event_sender: Arc<Mutex<mpsc::Sender<Event<'static, ()>>>>,
    platform: Arc<Mutex<Platform>>,
    ctx: Arc<Mutex<CtxRef>>,
}

impl NodiumViewEgui {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let platform = Platform::new(PlatformDescriptor {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor(),
            font_definitions: Default::default(),
            style: Default::default(),
        });
        let ctx = CtxRef::default();

        let (event_sender, event_receiver) = mpsc::channel();

        std::thread::spawn(move || {
            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => (),
                    },
                    Event::RedrawRequested(_) => {
                        let mut platform = platform.lock().unwrap();
                        platform.begin_frame();
                        let mut ctx = ctx.lock().unwrap();

                        // Here, you can define your EGUI UI and handle events

                        let (output, paint_commands) = platform.end_frame();
                        platform.paint(paint_commands);
                    }
                    _ => (),
                }

                platform.lock().unwrap().handle_event(&event);
                let _ = event_sender.send(event.to_static());
            });
        });

        NodiumViewEgui {
            event_sender: Arc::new(Mutex::new(event_sender)),
            platform: Arc::new(Mutex::new(platform)),
            ctx: Arc::new(Mutex::new(ctx)),
        }
    }
}

impl NodiumView for NodiumViewEgui {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // The UI thread is already running in the background. You can handle events received from the channel here.
        Ok(())
    }

    fn add_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn remove_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
