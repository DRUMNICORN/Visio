// ignore unused rust
#![allow(unused_imports)]

use std::sync::Arc;
use tokio::sync::{mpsc::channel, Mutex, MutexGuard};

use log::debug;

use egui::{CtxRef, Label, Rgba};
use egui_winit_platform::{Platform, PlatformDescriptor};
use nodium_app::NodiumView;
use nodium_pdk::{NodiumEvent, NodiumLayout, NodiumNode, NodiumWindow};
use serde_json::{de, from_str, to_value};
use tokio::sync::mpsc::Sender;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Clone)]
pub struct NodiumViewEgui {
    event_sender: Arc<Mutex<Sender<Option<Event<'static, ()>>>>>,
    event_loop: Arc<Mutex<EventLoop<()>>>,
    platform: Arc<Mutex<Platform>>,
    ctx: Arc<egui::CtxRef>,
    sender: Sender<Event<'static, ()>>,
}

use async_trait::async_trait;

impl NodiumViewEgui {
    pub fn new() -> Self {
        let event_loop = Arc::new(Mutex::new(EventLoop::new()));
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let platform = Arc::new(Mutex::new(Platform::new(PlatformDescriptor {
            physical_width: window.inner_size().width,
            physical_height: window.inner_size().height,
            scale_factor: window.scale_factor(),
            font_definitions: Default::default(),
            style: Default::default(),
        })));
        let ctx = Arc::new(Mutex::new(CtxRef::default()));

        let (sender, event_receiver) = channel(100);

        let event_sender: Sender<Option<Event<'static, ()>>> =
            unsafe { std::mem::transmute(sender) };

        NodiumViewEgui {
            event_sender: Arc::new(Mutex::new(event_sender)),
            event_loop,
            platform,
            ctx,
            sender,
        }
    }

    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_loop = self.event_loop.lock().await;
        let mut event_receiver = self.event_sender.lock().await;
        let event_loop_proxy = event_loop.create_proxy();
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                Event::RedrawRequested(id) => {
                    tokio::spawn(async move {
                        let platform_a = self.platform.lock().await;
                        let ctx_a = self.ctx.lock().await;
                        let event_loop_proxy = event_loop_proxy.clone();
                        let mut platform_a: MutexGuard<Platform> = platform_a;
                        let mut ctx_a = ctx_a;
                        platform_a.begin_frame();

                        // Define the EGUI UI here
                        egui::CentralPanel::default().show(&ctx_a, |ui| {
                            ui.add(Label::new("Hello, world!"));
                        });

                        let (output, paint_commands) = platform_a.end_frame();

                        platform_a.handle_event(&event);
                        debug!("paint_commands: {:?}", paint_commands);

                        // TODO: send paint_commands to nodium
                    });
                }
                Event::UserEvent(()) => {
                    let _ = self.sender.send(event.to_static());
                }
                _ => (),
            }
        });
    }
}

#[async_trait]
impl NodiumView for NodiumViewEgui {
    fn add_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("add_window");
        Ok(())
    }

    fn focus_window(
        &self,
        _: Box<dyn NodiumWindow + 'static>,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        debug!("focus_window");
        Ok(())
    }

    fn remove_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("remove_window");
        Ok(())
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_loop = self.event_loop.lock().await;
        self.start().await?;
        Ok(())
    }

    fn set_layout(&self, _: NodiumLayout) -> Result<(), Box<dyn std::error::Error + 'static>> {
        debug!("set_layout");
        Ok(())
    }

    fn update_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("update_window");
        Ok(())
    }
}
