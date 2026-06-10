use std::collections::HashMap;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Default,
}
pub struct Shader {
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pipelines: HashMap<ShaderType, wgpu::RenderPipeline>,
}
impl Shader {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        vertex_layout: wgpu::VertexBufferLayout<'static>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let uniform_bind_group_layout = Self::create_uniform_layout(&device);
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shader Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shader Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: color_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut pipelines = HashMap::new();
        pipelines.insert(ShaderType::Default, pipeline);
        Self {
            uniform_bind_group_layout,
            pipelines,
        }
    }
    pub fn get(&self, shader_type: ShaderType) -> Option<&wgpu::RenderPipeline> {
        self.pipelines.get(&shader_type)
    }
    fn create_uniform_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shader Uniform Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}
