use crate::types::Ray;

use super::shading_context::ShadingContext;

pub trait RayShader {
    fn shade(&self, ctx: &ShadingContext) -> Ray;
}
