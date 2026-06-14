use crate::shader::ShaderType;
use crate::texture_library::TextureLibrary;
use crate::{material::Material, texture_library::TextureType};
use std::{collections::HashMap, sync::Arc};
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum MaterialType {
    #[default]
    None,
    Test,
}
struct MaterialLibrary {
    materials: HashMap<MaterialType, Arc<Material>>,
    texture_library: TextureLibrary,
}
impl MaterialLibrary {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, queue: &wgpu::Queue) -> Self {
        let texture_library = TextureLibrary::new(device, queue);
        let mut materials = HashMap::new();
        if let Some(texture) = texture_library.get(&TextureType::Test) {
            materials.insert(
                MaterialType::Test,
                Arc::new(Material::new(
                    &device,
                    &layout,
                    texture.clone(),
                    ShaderType::Default,
                )),
            );
        }
        Self {
            materials,
            texture_library,
        }
    }
}
