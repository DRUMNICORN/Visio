
#[derive(Debug, Clone)]
pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_keyboard_input(&self, input: winit::event::KeyboardInput, flow: std::sync::Arc<std::sync::Mutex<crate::flow::Flow>>) {
        // Handle keyboard input and update the flow accordingly.
        log::debug!("Handling keyboard input: {:?}", input);
        let flow = match flow.lock() {
            Ok(flow) => flow,
            Err(_) => panic!("Failed to lock flow"),
        };
        
        log::debug!("Flow: {:?}", flow);

        match input {
            winit::event::KeyboardInput {
                state: winit::event::ElementState::Pressed,
                virtual_keycode: Some(winit::event::VirtualKeyCode::A),
                ..
            } => {
                log::debug!("Pressed A");
            }
            _ => {}
        }
    }
}
