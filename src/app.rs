use std::sync::{Arc, Mutex};

// app.rs
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{input_handler::InputHandler, renderer::Renderer, flow::Flow};

pub struct App {
    input_handler: InputHandler,
    flow: Arc<Mutex<Flow>>,
}

impl App {
    pub fn new() -> Self {

        let input_handler = InputHandler::new();

        let flow = Arc::new(Mutex::new(Flow::new()));

        Self {
            input_handler,
            flow,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = Renderer::new(&window);

        // Clone the Arc<Mutex<Flow>> to avoid moving it into the closure
        let flow = self.flow.clone();
        let window_id = window.id();
        let input_handler = self.input_handler.clone(); // assuming InputHandler implements Clone
    
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    window_id: id,
                } if id == window_id => {
                    // Clone the Arc<Mutex<Flow>> to avoid moving it
                    let flow = flow.clone();
                    input_handler.handle_keyboard_input(input, flow);
                }
                Event::MainEventsCleared => {
                    // Clone the Arc<Mutex<Flow>> to avoid moving it
                    let flow = flow.clone();
                    // update function needs to be adjusted to be a standalone function or a method of another object
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // Clone the Arc<Mutex<Flow>> to avoid moving it
                    let flow = flow.clone();
                    renderer.render(flow);
                }
                _ => (),
            }
        });
    }

    fn update(&mut self, flow: Arc<Mutex<Flow>>) {
        // Application update code.
        let flow = match flow.lock() {
            Ok(flow) => flow,
            Err(_) => panic!("Failed to lock flow"),
        };

        log::debug!("Flow: {:?}", flow);
    }
}