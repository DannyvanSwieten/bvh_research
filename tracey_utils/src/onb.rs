use libraytracer::types::{Direction, Vec3};

pub struct OrthoNormalBasis {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthoNormalBasis {
    pub fn new(n: &Direction) -> Self {
        let w = *n;
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        Self { u, v, w }
    }

    pub fn to_local(&self, v: &Direction) -> Direction {
        self.u * v.x + self.v * v.y + self.w * v.z
    }
}
