use crate::ecs::World;
use crate::primitive_mesh::PrimitiveMesh;
use crate::shader::{Shader, ShaderType};
use crate::texture_library::TextureLibrary;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    shader: Shader,
    primitive_mesh: PrimitiveMesh,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
}
impl Renderer {
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
        let primitive_mesh = PrimitiveMesh::new(&device);
        Self {
            surface,
            device,
            queue,
            config,
            shader,
            primitive_mesh,
            depth_texture,
            depth_view,
        }
    }
    pub fn render(&mut self, world: &World) {
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
            let view_proj = cam.get_matrix(
                self.config.width,
                self.config.height,
                cam_trans.get_matrix(),
            );
            let load_op = if cam.is_clear {
                wgpu::LoadOp::Clear(cam.clear_color)
            } else {
                wgpu::LoadOp::Load
            };
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: load_op,
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
            for (((trans_op, prim_type_op), material_op), layer_op) in world
                .transforms
                .iter()
                .zip(world.primitive_type.iter())
                .zip(world.materials.iter())
                .zip(world.layers.iter())
            {
                let (Some(trans), Some(prim), Some(material), Some(layer)) =
                    (trans_op, prim_type_op, material_op, layer_op)
                else {
                    continue;
                };
                if *layer != cam.culling_mask {
                    continue;
                }
                if let Some(pipeline) = self.shader.get(material.shader_type) {
                    render_pass.set_pipeline(&pipeline);
                }
                let mvp = view_proj * trans.get_matrix();
                material.update_matrix(&self.queue, mvp);
                material.bind(&mut render_pass);
                if let Some(mesh) = self.primitive_mesh.get(*prim) {
                    mesh.render(&mut render_pass);
                }
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        ouput.present();
    }
    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        (self.depth_texture, self.depth_view) =
            Self::create_depth_texture(&self.device, self.config.width, self.config.height);
    }
    pub fn setup_world(&mut self, world: &mut World) {
        let texture_library = TextureLibrary::new(&self.device, &self.queue);
        for (entity, ((trans_op, prim_op), tex_op)) in world
            .transforms
            .iter()
            .zip(world.primitive_type.iter())
            .zip(world.texture_type.iter())
            .enumerate()
        {
            if trans_op.is_some() && prim_op.is_some() && tex_op.is_some() {
                let tex_type = tex_op.unwrap();
                let texture = texture_library.get(&tex_type).unwrap().clone();
                let shader_type = ShaderType::Default;
                let material = crate::material::Material::new(
                    &self.device,
                    &self.shader.uniform_bind_group_layout,
                    texture,
                    shader_type,
                );
                world.materials.set(entity, material);
            }
        }
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
