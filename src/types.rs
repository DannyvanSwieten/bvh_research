use std::{
    ops::{Add, Index},
    simd::{Simd, SimdElement},
};

pub struct Vector3<T: std::simd::SimdElement> {
    data: Simd<T, 4>,
}

impl<T: std::simd::SimdElement> From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        Self {
            data: Simd::from([value, value, value, value]),
        }
    }
}

impl<T: std::simd::SimdElement + Default> From<[T; 3]> for Vector3<T> {
    fn from(value: [T; 3]) -> Self {
        Self {
            data: Simd::from([value[0], value[1], value[2], T::default()]),
        }
    }
}

impl<T: std::simd::SimdElement + Default> From<Simd<T, 4>> for Vector3<T> {
    fn from(value: Simd<T, 4>) -> Self {
        Self { data: value }
    }
}

impl<T: std::simd::SimdElement> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: SimdElement> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.data + &rhs.data)
    }
}

pub type Vec3 = Vector3<f32>;
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

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::from([x, y, z])
}

pub fn min(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    vec3(lhs[0].min(rhs[0]), lhs[1].min(rhs[1]), lhs[2].min(rhs[2]))
}
pub fn max(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    vec3(lhs[0].max(rhs[0]), lhs[1].max(rhs[1]), lhs[2].max(rhs[2]))
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
        Vec3::from(self.max.data - self.min.data)
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
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: vec3(f32::MAX, f32::MAX, f32::MAX),
            max: vec3(f32::MIN, f32::MIN, f32::MIN),
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
            inv_direcion: Vec3::from(1.0) / direction,
        }
    }
}
