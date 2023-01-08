use cgmath::InnerSpace;

use crate::types::{Position, Ray};

pub struct Camera {
    position: Position,
    p0: Position, // top-left
    p1: Position, // top-right
    p2: Position, // bottom-right
}

impl Camera {
    pub fn new(position: Position, z: f32) -> Self {
        Self {
            position,
            p0: Position::new(-1.0, 1.0, z),
            p1: Position::new(1.0, 1.0, z),
            p2: Position::new(-1.0, -1.0, z),
        }
    }

    pub fn ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let pixel_position = self.p0
            + (self.p1 - self.p0) * (x as f32 / width as f32)
            + (self.p2 - self.p0) * (y as f32 / height as f32);

        let direction = (pixel_position - self.position).normalize();
        Ray::new(self.position, direction)
    }
}
