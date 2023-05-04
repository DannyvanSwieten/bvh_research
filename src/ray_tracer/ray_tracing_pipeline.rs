use std::rc::Rc;

use vk_utils::device_context::DeviceContext;

use crate::gpu::{gpu_ray_generator::GpuRayGenerator, gpu_ray_shader::GpuRayShader};

use super::{
    shader_binding_table::ShaderBindingTable, shader_compiler::ShaderCompiler,
    shader_module::ShaderModule,
};

pub struct RayTracingPipeline {
    ray_generator: GpuRayGenerator,
    ray_shader: GpuRayShader,
}

impl RayTracingPipeline {
    // pub fn new(shader_binding_table: ShaderBindingTable) -> Self {
    //     let compiler = ShaderCompiler::new();
    //     compiler.compile(&shader_binding_table);

    //     Self {
    //         shader_binding_table,
    //     }
    // }

    pub fn new(
        device: Rc<DeviceContext>,
        ray_gen_module: &ShaderModule,
        miss_module: &ShaderModule,
        hit_module: &ShaderModule,
    ) {
        let ray_generator =
            GpuRayGenerator::new_from_string(device.clone(), ray_gen_module.source(), 1, None);
        let ray_shader = GpuRayShader::new_from_string(device, ray_gen_module.source(), 1, None);
        // let shader_binding_table = ShaderBindingTable::new(ray_gen_module, miss_module, hit_module);
    }

    pub fn shader_binding_table(&self) -> &ShaderBindingTable {
        &self.shader_binding_table
    }
}
