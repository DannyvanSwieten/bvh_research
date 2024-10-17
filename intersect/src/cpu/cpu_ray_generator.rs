use crate::types::{Ray, Vec2};

pub trait RayGenerator {
    fn generate(&self, pixel: Vec2, resolution: Vec2) -> Ray;
}
