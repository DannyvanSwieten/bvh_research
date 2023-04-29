use std::rc::Rc;

use crate::gpu::gpu_ray_generator::GpuRayGenerator;

use super::shader::{HitShader, IntersectionShader, MissShader};

pub struct ShaderBindingTable {
    ray_generation_shader: Rc<GpuRayGenerator>,
    ray_hit_shaders: Vec<Rc<dyn HitShader>>,
    intersection_shaders: Vec<Rc<dyn IntersectionShader>>,
    miss_shader: Option<Rc<dyn MissShader>>,
}

impl ShaderBindingTable {
    pub fn new(ray_generation_shader: Rc<GpuRayGenerator>) -> Self {
        Self {
            ray_generation_shader,
            ray_hit_shaders: Vec::new(),
            intersection_shaders: Vec::new(),
            miss_shader: None,
        }
    }

    pub fn add_hit_shader(&mut self, shader: Rc<dyn HitShader>) {
        self.ray_hit_shaders.push(shader);
    }

    pub fn add_intersection_shader(&mut self, shader: Rc<dyn IntersectionShader>) {
        self.intersection_shaders.push(shader);
    }

    pub fn ray_generation_shader(&self) -> &Rc<GpuRayGenerator> {
        &self.ray_generation_shader
    }

    pub fn ray_hit_shaders(&self) -> &Vec<Rc<dyn HitShader>> {
        &self.ray_hit_shaders
    }

    pub fn set_miss_shader(&mut self, shader: Rc<dyn MissShader>) {
        self.miss_shader = Some(shader);
    }
}
