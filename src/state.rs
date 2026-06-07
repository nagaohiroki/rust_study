use crate::input_manager::InputManager;
use crate::scene::Scene;
use crate::shader::Shader;
use crate::shader_uniform::ShaderUniform;
use crate::time_manager::TimeManager;
use std::sync::Arc;
use wgpu;
use winit;
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub input: InputManager,
    pub time: TimeManager,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    shader: Shader,
    uniform: ShaderUniform,
    scene: Scene,
}
impl State {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let sufface_caps = surface.get_capabilities(&adapter);
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: sufface_caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: sufface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let (depth_texture, depth_view) =
            Self::create_depth_texture(&device, config.width, config.height);
        let shader = Shader::new(
            &device,
            config.format,
            wgpu::TextureFormat::Depth32Float,
            crate::mesh::Vertex::desc(),
        );
        let uniform = ShaderUniform::new(&device);
        let scene = Scene::create_test(&device);
        Self {
            surface,
            device,
            queue,
            config,
            input: InputManager::new(),
            time: TimeManager::new(),
            depth_texture,
            depth_view,
            shader,
            uniform,
            scene,
        }
    }
    pub fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        (self.depth_texture, self.depth_view) =
            Self::create_depth_texture(&self.device, self.config.width, self.config.height);
    }
    pub fn input_event(&mut self, key_event: &winit::event::KeyEvent) {
        if let winit::keyboard::PhysicalKey::Code(keycode) = key_event.physical_key {
            let is_pressed = key_event.state == winit::event::ElementState::Pressed;
            self.input.handle_event(keycode, is_pressed);
        }
    }
    pub fn update(&mut self) {
        self.time.update();
        self.scene.update(&self.input, &self.time);
        self.input.update(self.time.delta_time());
        self.render_system();
    }
    fn render_system(&mut self) {
        let world = &self.scene.world;
        let ouput = self.surface.get_current_texture().unwrap();
        let view = ouput
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        for (cam_trans_op, cam_op) in world.transforms.iter().zip(world.cameras.iter()) {
            let (Some(cam_trans), Some(cam)) = (cam_trans_op, cam_op) else {
                continue;
            };
            let cam_mat = cam.get_matrix(self.config.width, self.config.height);
            let view_proj = cam_mat * cam_trans.get_matrix();
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.shader.bind(&mut render_pass);
            for (trans_op, mesh_op) in world.transforms.iter().zip(world.meshes.iter()) {
                let (Some(trans), Some(mesh)) = (trans_op, mesh_op) else {
                    continue;
                };
                let mvp = view_proj * trans.get_matrix();
                self.uniform.bind_matrix(&self.queue, &mut render_pass, mvp);
                mesh.render(&mut render_pass);
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        ouput.present();
    }
    fn create_depth_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let depth_format = wgpu::TextureFormat::Depth32Float;
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: depth_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        (depth_texture, depth_view)
    }
}
