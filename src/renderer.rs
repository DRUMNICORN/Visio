use std::sync::{Mutex, Arc};


#[derive(Clone, Debug)]
pub struct Renderer {
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    surface: Arc<Mutex<wgpu::Surface>>,
    // Add more rendering-related fields as needed.
}

impl Renderer {
    pub fn new(window: &winit::window::Window) -> Self {
        // Initialize renderer components.

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        // Create device, queue, and surface.
        let surface = match unsafe { instance.create_surface(&window) } {
            Ok(surface) => surface,
            Err(_) => panic!("Failed to create surface"),
        };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        // Replace this with code that initializes and configures the renderer.
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None, // Trace path
        ))
        .unwrap();

        // Configure rendering resources.

        Self {
            device: Arc::new(Mutex::new(device)),
            queue: Arc::new(Mutex::new(queue)),
            surface: Arc::new(Mutex::new(surface)),
        }
    }

    pub fn render(&self, flow: std::sync::Arc<std::sync::Mutex<crate::flow::Flow>>) {
        // Update rendering resources as needed.
        let flow = match flow.lock() {
            Ok(flow) => flow,
            Err(_) => panic!("Failed to lock flow"),
        };
        log::debug!("Flow: {:?}", flow);
        // Render the flow.

        
    }
}