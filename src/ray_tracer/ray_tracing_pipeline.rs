use std::rc::Rc;

use vk_utils::device_context::DeviceContext;

use crate::gpu::{
    gpu_ray_accumulator::GpuRayAccumulator, gpu_ray_generator::GpuRayGenerator,
    gpu_ray_shader::GpuRayShader,
};

use super::{shader_binding_table::ShaderBindingTable, shader_module::ShaderModule};

pub struct RayTracingPipeline {
    ray_generator: GpuRayGenerator,
    hit_shader: GpuRayShader,
    miss_shader: GpuRayShader,
    ray_accumulator: GpuRayAccumulator,
}

impl RayTracingPipeline {
    // pub fn new(shader_binding_table: ShaderBindingTable) -> Self {
    //     let compiler = ShaderCompiler::new();
    //     compiler.compile(&shader_binding_table);

    //     Self {
    //         shader_binding_table,
    //     }
    // }

    pub fn new(device: Rc<DeviceContext>, shader_binding_table: ShaderBindingTable) -> Self {
        let ray_generator = GpuRayGenerator::new(
            device.clone(),
            &shader_binding_table.ray_generation_shader(),
            1,
            None,
        );
        let hit_shader = GpuRayShader::new_from_string(
            device.clone(),
            shader_binding_table.ray_hit_shaders()[0].source(),
            1,
            None,
        );
        let miss_shader = GpuRayShader::new_from_string(
            device.clone(),
            shader_binding_table.miss_shader().unwrap().source(),
            1,
            None,
        );
        let ray_accumulator = GpuRayAccumulator::new(device.clone(), 1);
        Self {
            ray_generator,
            hit_shader,
            miss_shader,
            ray_accumulator,
        }
    }
}
