use crate::{
    bottom_level_acceleration_structure::AccelerationStructure,
    intersect::intersect_triangle,
    types::{vec4_to_3, Mat4, Ray, Triangle, Vertex, AABB},
};

pub struct BruteForceStructure {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    aabb: AABB,
}

impl BruteForceStructure {
    pub fn new(vertices: &[Vertex], triangles: &[Triangle]) -> Self {
        let mut aabb = AABB::default();
        for t in triangles {
            aabb.grow_with_position(&vec4_to_3(vertices[t.v0 as usize]));
            aabb.grow_with_position(&vec4_to_3(vertices[t.v1 as usize]));
            aabb.grow_with_position(&vec4_to_3(vertices[t.v2 as usize]));
        }
        Self {
            vertices: vertices.to_vec(),
            triangles: triangles.to_vec(),
            aabb,
        }
    }
}

impl AccelerationStructure for BruteForceStructure {
    fn trace(&self, ray: &Ray, transform: &Mat4) -> (i32, f32) {
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

    fn aabb(&self) -> &crate::types::AABB {
        todo!()
    }
}
