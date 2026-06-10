use crate::texture::Texture;
use std::collections::HashMap;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum TextureType {
    #[default]
    None,
    Test,
}
pub struct TextureLibrary {
    data: HashMap<TextureType, Texture>,
}
impl TextureLibrary {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut data = HashMap::new();
        let test = Texture::from_bytes(&device, &queue, include_bytes!("image.png"), "image.png");
        data.insert(TextureType::Test, test);
        let none: [u8; 4] = [255, 255, 255, 255];
        let none_tex = Texture::from_pixels(&device, &queue, &none, 1, 1, "none");
        data.insert(TextureType::None, none_tex);
        Self { data }
    }
    pub fn get(&self, texture_type: &TextureType) -> Option<&Texture> {
        self.data.get(texture_type)
    }
}
