use wgpu::util::DeviceExt;
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub mvp: glam::Mat4,
}
pub struct ShaderUniform {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}
impl ShaderUniform {
    pub fn new(device: &wgpu::Device, uniform_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let data = Uniforms {
            mvp: glam::Mat4::IDENTITY,
        };
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self { buffer, bind_group }
    }
    pub fn update_matrix(&self, queue: &wgpu::Queue, matrix: glam::Mat4) {
        let data = Uniforms { mvp: matrix };
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&data));
    }
    pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
}
