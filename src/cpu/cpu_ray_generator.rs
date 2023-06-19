use crate::{top_level_acceleration_structure::TopLevelAccelerationStructure, types::Vec2};

use super::cpu_shader_binding_table::ShaderBindingTable;

pub trait RayGenerationShader<Context, Payload> {
    fn execute(
        &self,
        ctx: &Context,
        tlas: &TopLevelAccelerationStructure,
        sbt: &ShaderBindingTable<Context, Payload>,
        payload: &mut Payload,
        pixel: &Vec2,
        resolution: &Vec2,
    );
}
