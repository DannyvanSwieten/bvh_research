use cgmath::{InnerSpace, Transform};

use crate::types::{Mat4, Position, Ray, Vec2, Vec4};

pub struct Camera {
    inv_view: Mat4,
    inv_proj: Mat4,
}

impl Camera {
    pub fn new(position: Position, fov: f32, resolution: &Vec2) -> Self {
        let inv_view = Mat4::from_translation(position)
            .inverse_transform()
            .unwrap();

        let inv_proj =
            cgmath::perspective(cgmath::Deg(fov), resolution.x / resolution.y, 0.01, 100.0)
                .inverse_transform()
                .unwrap();

        Self { inv_view, inv_proj }
    }

    pub fn ray(&self, location: &Vec2, resolution: &Vec2) -> Ray {
        let mut st = *location;
        st.x /= resolution.x;
        st.y /= resolution.y;
        let mut st = st * 2.0 - Vec2::new(1.0, 1.0);
        st.y = -st.y;

        let pixel_position = (self.inv_proj * Vec4::new(st.x, st.y, 1.0, 1.0)).truncate();
        let origin = (self.inv_view * Vec4::new(0.0, 0.0, 0.0, 1.0)).truncate();

        let direction = (pixel_position - origin).normalize();
        let direction =
            (self.inv_view * Vec4::new(direction.x, direction.y, direction.z, 0.0)).truncate();
        Ray::new(origin, direction)
    }
}
