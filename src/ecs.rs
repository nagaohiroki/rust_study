use crate::camera::Camera;
use crate::primitive::PrimitiveType;
use crate::shader_uniform::ShaderUniform;
use crate::transform::Transform;
pub struct Component<T> {
    data: Vec<Option<T>>,
}
impl<T> Component<T> {
    pub fn set(&mut self, entity: usize, component: T) {
        if entity >= self.data.len() {
            self.data.resize_with(entity + 1, || None);
        }
        self.data[entity] = Some(component);
    }
    pub fn get_mut(&mut self, entity: usize) -> Option<&mut T> {
        self.data.get_mut(entity)?.as_mut()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Option<T>> {
        self.data.iter()
    }
}
impl<T> Default for Component<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
#[derive(Default)]
pub struct World {
    pub transforms: Component<Transform>,
    pub cameras: Component<Camera>,
    pub uniforms: Component<ShaderUniform>,
    pub primitive_type: Component<PrimitiveType>,
    next_entity_id: usize,
}
impl World {
    pub fn create_entity(&mut self) -> usize {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }
}
