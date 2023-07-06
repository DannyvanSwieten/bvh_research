use libraytracer::cpu::camera::Camera;
use tracey_utils::sampler::Sampler;

use crate::scene::Scene;

pub struct Ctx {
    pub camera: Camera,
    pub scene: Scene,
    pub sampler: Box<dyn Sampler>,
    pub spp: usize,
}

unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}
