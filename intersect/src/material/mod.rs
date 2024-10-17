use crate::types::HdrColor;

use self::material_context::MaterialContext;

pub mod material_compiler;
pub mod material_context;
pub mod shader;
pub mod shader_module;
pub trait Material {
    fn upload(&self, ctx: &mut MaterialContext);
    fn shader(&self) -> usize;
}

pub struct DiffuseMaterial {
    pub shader: usize,
    pub color: HdrColor,
}

impl DiffuseMaterial {
    pub fn new(shader: usize, color: HdrColor) -> Self {
        Self { shader, color }
    }
}

impl Material for DiffuseMaterial {
    fn upload(&self, ctx: &mut MaterialContext) {
        ctx.push_float4(self.color);
    }

    fn shader(&self) -> usize {
        self.shader
    }
}
