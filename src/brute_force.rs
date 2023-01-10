use crate::{
    acceleration_structure::AccelerationStructure,
    intersect::intersect_triangle,
    types::{Ray, Triangle, Vertex},
};

pub struct BruteForceStructure {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl BruteForceStructure {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }
}

impl Default for BruteForceStructure {
    fn default() -> Self {
        Self::new()
    }
}

impl AccelerationStructure for BruteForceStructure {
    fn build(&mut self, vertices: &[Vertex], triangles: &[Triangle]) {
        self.vertices = vertices.to_vec();
        self.triangles = triangles.to_vec();
    }

    fn trace(&self, ray: &Ray) -> (i32, f32) {
        let mut t = f32::MAX;
        for i in 0..self.triangles.len() {
            let triangle = &self.triangles[i];
            let v0 = self.vertices[triangle.v0 as usize];
            let v1 = self.vertices[triangle.v1 as usize];
            let v2 = self.vertices[triangle.v2 as usize];

            let d = intersect_triangle(&ray, &v0, &v1, &v2);
            if d < t {
                t = d;
            }
        }

        (0, t)
    }
}
