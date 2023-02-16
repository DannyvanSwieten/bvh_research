use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector3, Vector4};

pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
pub type Vertex = Vec4;
pub type Position = Vec3;
pub type Direction = Vec3;
pub type Origin = Vec3;
pub type HdrColor = Vec4;
pub type Mat4 = Matrix4<f32>;

pub fn vec4_to_3(v: Vec4) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct Triangle {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn min(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(lhs[0].min(rhs[0]), lhs[1].min(rhs[1]), lhs[2].min(rhs[2]))
}
pub fn max(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(lhs[0].max(rhs[0]), lhs[1].max(rhs[1]), lhs[2].max(rhs[2]))
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
        e[0] * e[1] + e[1] * e[2] + e[2] * e[0]
    }

    pub fn dominant_axis(&self) -> usize {
        let extent = self.extent();
        let mut axis = 0;
        if extent[1] > extent[0] {
            axis = 1;
        }
        if extent[2] > extent[axis] {
            axis = 2;
        }

        axis
    }

    pub fn centroid(&self) -> Vec3 {
        self.extent() * 0.5
    }

    pub fn transformed(&self, transform: &Mat4) -> Self {
        let min = transform * Vec4::new(self.min.x, self.min.y, self.min.z, 1.0);
        let max = transform * Vec4::new(self.max.x, self.max.y, self.max.z, 1.0);
        Self {
            min: min.truncate(),
            max: max.truncate(),
        }
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

#[repr(C)]
#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct Ray {
    pub origin: Origin,
    pub direction: Direction,
    pub inv_direcion: Direction,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Origin::new(0.0, 0.0, 0.0),
            direction: Direction::new(1.0, 1.0, 1.0),
            inv_direcion: Direction::new(1.0, 1.0, 1.0),
        }
    }
}

impl Ray {
    pub fn new(origin: Position, direction: Direction) -> Ray {
        Self {
            origin,
            direction,
            inv_direcion: 1.0 / direction,
        }
    }

    pub fn transformed(&self, transform: &Mat4) -> Self {
        let o = transform * Vec4::new(self.origin.x, self.origin.y, self.origin.z, 1.0);
        let d = transform * Vec4::new(self.direction.x, self.direction.y, self.direction.z, 0.0);
        Self::new(o.truncate(), d.truncate().normalize())
    }
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub ray: Ray,
    pub object_id: u32,
    pub primitive_id: u32,
    pub obj_to_world: Mat4,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: f32::MAX,
            u: 0.0,
            v: 0.0,
            ray: Ray::default(),
            object_id: 0,
            primitive_id: 0,
            obj_to_world: Mat4::identity(),
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
