use std::{collections::HashMap, sync::Arc};

use crate::{
    mesh::Mesh,
    primitive::{Primitive, PrimitiveType},
};
pub struct PrimitiveMesh {
    data: HashMap<PrimitiveType, Arc<Mesh>>,
}
impl PrimitiveMesh {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut data = HashMap::new();
        let (vertices, indices) = Primitive::quad();
        let mesh_quad = Arc::new(Mesh::new(&device, &vertices, &indices));
        data.insert(PrimitiveType::Quad, mesh_quad);
        Self { data }
    }
    pub fn get(&self, primitive_type: PrimitiveType) -> Option<&Arc<Mesh>> {
        self.data.get(&primitive_type)
    }
}
