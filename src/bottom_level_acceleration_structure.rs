use crate::types::{HitRecord, Mat4, Ray, AABB};

pub trait AccelerationStructure {
    fn trace(&self, ray: &Ray, transform: &Mat4, record: &mut HitRecord);
    fn aabb(&self) -> &AABB;
}
