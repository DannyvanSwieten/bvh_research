use crate::types::{Direction, HitRecord, Mat4, Position, Ray, RayType, Vec2, AABB};

use super::bvh::BottomLevelAccelerationStructure;

pub struct SurfaceAttributes {
    pub position: Position,
    pub normal: Direction,
    pub uv: Vec2,
}

pub trait Shape {
    fn intersect(
        &self,
        ray: &Ray,
        ray_type: RayType,
        transform: &Mat4,
        t_max: f32,
    ) -> Option<HitRecord>;
    fn surface_attributes(&self, hit_record: &HitRecord) -> SurfaceAttributes;
    fn aabb(&self) -> AABB;
}

// impl Shape for BottomLevelAccelerationStructure {
//     fn intersect(
//         &self,
//         ray: &Ray,
//         ray_type: RayType,
//         transform: &Mat4,
//         hit_record: &mut HitRecord,
//         t_max: f32,
//     ) {
//         self.traverse(ray, ray_type, transform, hit_record);
//     }

//     fn aabb(&self) -> AABB {
//         *self.aabb()
//     }

//     fn surface_attributes(&self, hit_record: &HitRecord) -> SurfaceAttributes {
//         todo!()
//     }
// }
