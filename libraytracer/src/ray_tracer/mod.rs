use self::ray_tracing_pipeline::RayTracingPipeline;

pub mod ray_tracing_pipeline;
pub mod shader;
pub mod shader_binding_table;
pub mod shader_compiler;
pub mod shader_module;
pub struct GpuRayTracer {}
impl GpuRayTracer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn trace(&self, _pipeline: &RayTracingPipeline) {
        unimplemented!()
    }
}
