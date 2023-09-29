// app.rs
use std::sync::{Arc, Mutex};

use nodium_controller::input_handler::InputHandler;
use nodium_flow::{
    flow::Flow,
    node::Node,
};
use nodium_view::renderer::Renderer;
use nodium_view::winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct App {
    input_handler: InputHandler,
    flow: Arc<Mutex<Flow>>,
}

impl App {
    pub fn new() -> Self {

        let input_handler = InputHandler::new();

        let flow = Arc::new(Mutex::new(Flow::new()));
        let mut app = Self {
            input_handler,
            flow,
        };

        app.generate_example_flows();

        app
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

    pub fn generate_example_flows(&mut self) {
        // create root node and randamly decide how many children it should have
        // and do this 4 times

        let mut flow = self.flow.lock().unwrap();

        let root_node = Node::new((0.0, 0.0));
        let root_node = flow.insert_node(root_node);
        
        fn create_child_nodes(flow: &mut Flow, parent_node_id: u64, depth: u64) {
            if depth > 0 {
                let num_children = rand::random::<u64>() % 4;
                for _ in 0..num_children {
                    let node = Node::new(((rand::random::<u64>() % 100) as f32, (rand::random::<u64>() % 100) as f32));
                    let child_node = flow.insert_node(node);
                    create_child_nodes(flow, parent_node_id+1, depth - 1);
                }
            }
        }

        create_child_nodes(&mut flow, 0, 4);

        log::debug!("Flow: {:?}", flow);
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