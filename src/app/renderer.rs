use std::{borrow::Cow, sync::Arc};

use wgpu::{ShaderModule, ShaderSource, TextureFormat};

use winit::{
    window::Window,
    dpi::PhysicalSize,
};
pub struct State <'window_state> {
    surface: wgpu::Surface<'window_state>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'window_state> State<'window_state> {
    pub async fn new_async(window: Arc<Window>) -> State<'window_state> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance
        .request_adapter(&adapter_descriptor)
        .await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor { 
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device")
        };

        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .expect("Failed to create device");

        let size  = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let shader_descripter = wgpu::ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/shader.wgsl"))),
        };

        let shader =device.create_shader_module(shader_descripter);

        let render_pipeline = create_pipeline(&device, &shader, surface_config.format);

        Self {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline,
        }
    }

    pub fn new(window: Arc<Window>) -> State<'window_state> {
        pollster::block_on(State::new_async(window))
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&self){
        let surface_texture = self
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut r_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.35,
                            g: 0.35,
                            b: 0.35,
                            a: 1.0
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            r_pass.set_pipeline(&self.render_pipeline);
            r_pass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    shader: &ShaderModule,
    swap_chain_format: TextureFormat,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(swap_chain_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            // topology: wgpu::PrimitiveTopology::TriangleList,
            // strip_index_format: None,
            // front_face: wgpu::FrontFace::Ccw,
            // cull_mode: Some(wgpu::Face::Back),
            // // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // // or Features::POLYGON_MODE_POINT
            // polygon_mode: wgpu::PolygonMode::Fill,
            // // Requires Features::DEPTH_CLIP_CONTROL
            // unclipped_depth: false,
            // // Requires Features::CONSERVATIVE_RASTERIZATION
            // conservative: false,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}