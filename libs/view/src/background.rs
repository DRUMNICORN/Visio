use std::sync::{Arc, Mutex};

use nodium_flow::flow::Flow;

pub struct BackgroundRenderer {
    device: Arc<Mutex<wgpu::Device>>,
    queue: Arc<Mutex<wgpu::Queue>>,
    surface: Arc<Mutex<wgpu::Surface>>,
    // Add more background rendering-related fields as needed.
}

impl BackgroundRenderer {
    pub fn new(device: Arc<Mutex<wgpu::Device>>, queue: Arc<Mutex<wgpu::Queue>>, surface: Arc<Mutex<wgpu::Surface>>) -> Self {
        Self {
            device,
            queue,
            surface,
            // Initialize other background rendering components here.
        }
    }

    pub fn render(&self, flow: Arc<Mutex<Flow>>) {
        // Render the background here.
        let device = self.device.lock().unwrap();
        let queue = self.queue.lock().unwrap();
        let surface = self.surface.lock().unwrap();

        let output = surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Background Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Background Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
