use std::sync::{Arc, Mutex};
use nodium_flow::flow::Flow;
use wgpu::{Queue, TextureView};
use winit::window::Window;

use crate::{nodes::NodeRenderer, background::BackgroundRenderer};

pub struct Renderer {
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<Queue>>,
    surface: Arc<Mutex<wgpu::Surface>>,
    background_renderer: BackgroundRenderer, // Add this field
    node_renderer: NodeRenderer, // Add this field
    // Add more rendering-related fields as needed.
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        // Initialize renderer components.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        // Create device, queue, and surface.
        let surface = match unsafe { instance.create_surface(window) } {
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

        // set winow to fullscreen
        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        
        // get max possible resolution
        let max_size = window.current_monitor().unwrap().size();
        window.set_inner_size(max_size);


        // Configure the surface
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: max_size.width,
            height: max_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        };
        surface.configure(&device, &surface_config);

        // set window name to "Nodium"
        window.set_title("Nodium");

        let device = Arc::new(Mutex::new(device));
        let queue = Arc::new(Mutex::new(queue));
        let surface = Arc::new(Mutex::new(surface));

        // Create the BackgroundRenderer and NodeRenderer instances.
        let background_renderer = BackgroundRenderer::new(
            Arc::clone(&device),
            Arc::clone(&queue),
            Arc::clone(&surface),
        );

        let node_renderer = NodeRenderer::new(
            Arc::clone(&device),
            Arc::clone(&queue),
        );

        Self {
            device,
            queue,
            surface,
            background_renderer,
            node_renderer,
        }
    }

    pub fn render(&self, flow: Arc<Mutex<Flow>>) {
        let flow_clone = flow.clone();

        // Render the background.
        self.background_renderer.render(flow);

        // Render the nodes.
        self.node_renderer.render(flow_clone);

        // Rest of the rendering logic as needed...
    }
}
