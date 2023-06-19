use crate::types::Vec2;

use super::{
    cpu_shader_binding_table::ShaderBindingTable,
    top_level_acceleration_structure::TopLevelAccelerationStructure,
};

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
