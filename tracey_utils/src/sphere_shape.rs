use libraytracer::{
    cpu::shape::{Shape, SurfaceAttributes},
    types::{HitRecord, Mat4, Ray, RayType, Vec2, Vec3, Vec4, AABB},
};

pub struct SphereShape {
    pub radius: f32,
}

impl SphereShape {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Shape for SphereShape {
    fn intersect(
        &self,
        ray: &Ray,
        _ray_type: RayType,
        transform: &Mat4,
        t_max: f32,
    ) -> Option<HitRecord> {
        // sphere intersection
        let center = transform * Vec4::new(0.0, 0.0, 0.0, 1.0);
        let l = ray.origin - center.xyz();
        let a = ray.direction.dot(&ray.direction);
        let b = l.dot(&ray.direction) * 2.0;
        let c = l.dot(&l) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        let mut x0;
        let mut x1;
        if discriminant < 0.0 {
            return None;
        } else if discriminant == 0.0 {
            let v = -0.5 * b / a;
            x0 = v;
            x1 = v;
        } else {
            let q = if b > 0.0 {
                -0.5 * (b + discriminant.sqrt())
            } else {
                -0.5 * (b - discriminant.sqrt())
            };
            x0 = q / a;
            x1 = c / q;
        }

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
        }

        if x0 < 0.0 {
            x0 = x1;
            if x0 < 0.0 {
                return None;
            }
        }

        if x0 > t_max {
            return None;
        }

        let hit_record = HitRecord {
            t: x0,
            ray: *ray,
            ..Default::default()
        };
        Some(hit_record)
    }

    fn aabb(&self) -> AABB {
        AABB::new(
            Vec3::new(-self.radius, -self.radius, -self.radius),
            Vec3::new(self.radius, self.radius, self.radius),
        )
    }

    fn surface_attributes(&self, hit_record: &HitRecord) -> SurfaceAttributes {
        let position = hit_record.ray.origin + hit_record.ray.direction * hit_record.t;
        let origin = hit_record.obj_to_world * Vec4::new(0.0, 0.0, 0.0, 1.0);
        let normal = (position - origin.xyz()).normalize();
        let u = 0.5 + normal.x.atan2(normal.z) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - normal.y.asin() / std::f32::consts::PI;
        let uv = Vec2::new(u, v);
        SurfaceAttributes {
            position,
            normal,
            uv,
        }
    }
}
