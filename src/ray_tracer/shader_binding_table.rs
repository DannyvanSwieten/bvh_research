use std::rc::Rc;

use super::shader_module::ShaderModule;

pub struct ShaderBindingTable {
    ray_generation_shader: Rc<ShaderModule>,
    ray_hit_shaders: Vec<Rc<ShaderModule>>,
    intersection_shaders: Vec<Rc<ShaderModule>>,
    miss_shader: Option<Rc<ShaderModule>>,
}

impl ShaderBindingTable {
    pub fn new(ray_generation_shader: Rc<ShaderModule>) -> Self {
        Self {
            ray_generation_shader,
            ray_hit_shaders: Vec::new(),
            intersection_shaders: Vec::new(),
            miss_shader: None,
        }
    }

    pub fn add_hit_shader(&mut self, shader: Rc<ShaderModule>) {
        self.ray_hit_shaders.push(shader);
    }

    pub fn add_intersection_shader(&mut self, shader: Rc<ShaderModule>) {
        self.intersection_shaders.push(shader);
    }

    pub fn ray_generation_shader(&self) -> Rc<ShaderModule> {
        self.ray_generation_shader.clone()
    }

    pub fn ray_hit_shaders(&self) -> &[Rc<ShaderModule>] {
        &self.ray_hit_shaders
    }

    pub fn set_miss_shader(&mut self, shader: Rc<ShaderModule>) {
        self.miss_shader = Some(shader);
    }

    pub fn miss_shader(&self) -> Option<Rc<ShaderModule>> {
        self.miss_shader.clone()
    }
}
