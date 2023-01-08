use crate::types::{Ray, Triangle, Vertex};

pub trait AccelerationStructure {
    fn build(&mut self, vertices: &[Vertex], triangles: &[Triangle]);
    fn trace(&self, ray: &Ray) -> (i32, f32);
}
