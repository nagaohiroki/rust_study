use crate::mesh::Vertex;
pub struct Primitive;
impl Primitive {
    pub fn quad() -> (Vec<Vertex>, Vec<u16>) {
        let vertices = vec![
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 1.0, 0.0],
            },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        (vertices, indices)
    }
}
