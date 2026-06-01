use wgpu::util::DeviceExt;
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: glam::Mat4,
}
pub struct ShaderUniform {
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,
}
impl ShaderUniform {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniforms = Uniforms {
            mvp: glam::Mat4::IDENTITY,
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        Self {
            uniform_buffer,
            uniform_bind_group,
            uniforms,
        }
    }
    pub fn set_matrix(&mut self, matrix: glam::Mat4) {
        self.uniforms.mvp = matrix;
    }
    pub fn bind<'a>(&'a self, queue: &mut wgpu::Queue, render_pass: &mut wgpu::RenderPass<'a>) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&self.uniforms));
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
    }
}
