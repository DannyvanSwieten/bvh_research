use crate::{gpu::gpu_ray_intersector::IntersectionResult, scene::Scene, types::Ray};

pub struct ShadingContext<'a> {
    pub ray: &'a Ray,
    pub intersection: &'a IntersectionResult,
    pub scene: &'a Scene,
}
