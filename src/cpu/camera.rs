use nalgebra_glm::perspective;

use crate::types::{Mat4, Position, Ray, Vec2, Vec4};

pub struct Camera {
    inv_view: Mat4,
    inv_proj: Mat4,
}

impl Camera {
    pub fn new(position: Position, fov: f32, resolution: &Vec2) -> Self {
        let inv_view = Mat4::new_translation(&position).try_inverse().unwrap();

        let inv_proj = perspective(resolution.x / resolution.y, fov.to_radians(), 0.01, 100.0)
            .try_inverse()
            .unwrap();

        Self { inv_view, inv_proj }
    }

    pub fn ray(&self, location: &Vec2, resolution: &Vec2) -> Ray {
        let mut st = *location;
        st.x /= resolution.x;
        st.y /= resolution.y;
        let mut st = st * 2.0 - Vec2::new(1.0, 1.0);
        st.y = -st.y;

        let origin = (self.inv_view * Vec4::new(0.0, 0.0, 0.0, 1.0)).xyz();
        let target = (self.inv_proj * Vec4::new(st.x, st.y, 1.0, 1.0))
            .xyz()
            .normalize();
        let direction = self.inv_view * Vec4::new(target.x, target.y, target.z, 0.0);

        Ray::new(origin, direction.xyz())
    }
}
