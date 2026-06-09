use crate::mesh::Vertex;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Quad,
    Cube,
}
pub struct Primitive;
impl Primitive {
    pub fn quad() -> (Vec<Vertex>, Vec<u16>) {
        let vertices = vec![
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [1.0, 1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [1.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        (vertices, indices)
    }
    pub fn cube() -> (Vec<Vertex>, Vec<u16>) {
        let vertices = vec![
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [0.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [0.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.0, 1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16,
            17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
        ];

        (vertices, indices)
    }
}
