use super::{cpu_ray_generator::RayGenerationShader, cpu_ray_shader::ClosestHitShader};

pub struct ShaderBindingTable<Context, Payload> {
    ray_generation_shader: Box<dyn RayGenerationShader<Context, Payload>>,
    closest_hit_shaders: Vec<Box<dyn ClosestHitShader<Context>>>,
}

impl<Context, Payload> ShaderBindingTable<Context, Payload> {
    pub fn new(ray_generation_shader: Box<dyn RayGenerationShader<Context, Payload>>) -> Self {
        Self {
            ray_generation_shader,
            closest_hit_shaders: Vec::new(),
        }
    }

    pub fn add_ray_hit_shader(&mut self, shader: Box<dyn ClosestHitShader<Context>>) {
        self.closest_hit_shaders.push(shader);
    }

    pub fn closest_hit_shaders(&self) -> &Vec<Box<dyn ClosestHitShader<Context>>> {
        &self.closest_hit_shaders
    }

    pub fn closest_hit_shader(&self, index: usize) -> &dyn ClosestHitShader<Context> {
        self.closest_hit_shaders[index].as_ref()
    }

    pub fn ray_generation_shader(&self) -> &dyn RayGenerationShader<Context, Payload> {
        self.ray_generation_shader.as_ref()
    }
}

unsafe impl<Context, Payload> Send for ShaderBindingTable<Context, Payload> {}
unsafe impl<Context, Payload> Sync for ShaderBindingTable<Context, Payload> {}
