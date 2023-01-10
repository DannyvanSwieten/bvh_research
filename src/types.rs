use cgmath::Vector3;
use cgmath::Vector4;

pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
pub type Vertex = Vec3;
pub type Position = Vec3;
pub type Direction = Vec3;
pub type Origin = Vec3;
pub type HdrColor = Vec4;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
}

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn min(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(lhs.x.min(rhs.x), lhs.y.min(rhs.y), lhs.z.min(rhs.z))
}
pub fn max(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(lhs.x.max(rhs.x), lhs.y.max(rhs.y), lhs.z.max(rhs.z))
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn grow(&mut self, other: &AABB) {
        self.min = min(&self.min, &other.min);
        self.max = max(&self.max, &other.max);
    }

    pub fn grow_with_position(&mut self, position: &Vec3) {
        self.min = min(&self.min, position);
        self.max = max(&self.max, position);
    }

    pub fn extent(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn area(&self) -> f32 {
        let e = self.extent();
        e.x * e.y + e.y * e.z + e.z * e.x
    }

    pub fn dominant_axis(&self) -> usize {
        let extent = self.extent();
        let mut axis = 0;
        if extent.y > extent.x {
            axis = 1;
        }
        if extent.z > extent[axis] {
            axis = 2;
        }

        axis
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }
}

pub struct Ray {
    pub origin: Origin,
    pub direction: Direction,
    pub inv_direcion: Direction,
}

impl Ray {
    pub fn new(origin: Position, direction: Direction) -> Ray {
        Self {
            origin,
            direction,
            inv_direcion: 1.0 / direction,
        }
    }
}
