use super::{shader_binding_table::ShaderBindingTable, shader_compiler::ShaderCompiler};

pub struct RayTracingPipeline {
    shader_binding_table: ShaderBindingTable,
}

impl RayTracingPipeline {
    // pub fn new(shader_binding_table: ShaderBindingTable) -> Self {
    //     let compiler = ShaderCompiler::new();
    //     compiler.compile(&shader_binding_table);

    //     Self {
    //         shader_binding_table,
    //     }
    // }

    pub fn shader_binding_table(&self) -> &ShaderBindingTable {
        &self.shader_binding_table
    }
}
