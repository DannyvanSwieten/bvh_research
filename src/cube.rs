use crate::types::Vertex;

pub struct Cube {
    vertices: [Vertex; 24],
    indices: [u32; 36],
}

impl Cube {
    pub fn new() -> Self {
        let vertices = [
            //front
            Vertex::new(-1., -1., -1.),
            Vertex::new(1., -1., -1.),
            Vertex::new(-1., 1., -1.),
            Vertex::new(1., 1., -1.),
            //back
            Vertex::new(-1., -1., 1.),
            Vertex::new(1., -1., 1.),
            Vertex::new(-1., 1., 1.),
            Vertex::new(1., 1., 1.),
            //bottom
            Vertex::new(-1., -1., -1.),
            Vertex::new(1., -1., -1.),
            Vertex::new(-1., -1., 1.),
            Vertex::new(1., -1., 1.),
            //top
            Vertex::new(-1., 1., -1.),
            Vertex::new(1., 1., -1.),
            Vertex::new(-1., 1., 1.),
            Vertex::new(1., 1., 1.),
            // left
            Vertex::new(-1., -1., 1.),
            Vertex::new(-1., -1., -1.),
            Vertex::new(-1., 1., 1.),
            Vertex::new(-1., 1., -1.),
            // right
            Vertex::new(1., -1., 1.),
            Vertex::new(1., -1., -1.),
            Vertex::new(1., 1., 1.),
            Vertex::new(1., 1., -1.),
        ];

        let indices = [
            0, 1, 2, 1, 3, 2, 4, 5, 6, 5, 7, 6, 8, 9, 10, 9, 11, 10, 12, 13, 14, 13, 15, 14, 16,
            17, 18, 17, 19, 18, 20, 21, 22, 21, 23, 22,
        ];

        Self { vertices, indices }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}
