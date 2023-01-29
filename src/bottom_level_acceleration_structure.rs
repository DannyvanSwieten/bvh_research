use crate::types::{Mat4, Ray, AABB};

pub trait AccelerationStructure {
    fn trace(&self, ray: &Ray, transform: &Mat4) -> (i32, f32);
    fn aabb(&self) -> &AABB;
}
