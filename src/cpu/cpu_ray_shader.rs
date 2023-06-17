use crate::types::{HitRecord, Ray};

pub trait ClosestHitShader<Context> {
    fn execute(&self, ctx: &Context, record: &HitRecord) -> Ray;
}
