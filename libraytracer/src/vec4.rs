use std::{
    ops::{Add, AddAssign, Div, Index, Mul, Sub},
    simd::Simd,
};
#[derive(Clone)]
pub struct Vector4 {
    data: Simd<f32, 4>,
}

impl From<f32> for Vector4 {
    fn from(value: f32) -> Self {
        Self {
            data: Simd::splat(value),
        }
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(value: [f32; 4]) -> Self {
        Self {
            data: Simd::from([value[0], value[1], value[2], f32::default()]),
        }
    }
}

impl From<Simd<f32, 4>> for Vector4 {
    fn from(value: Simd<f32, 4>) -> Self {
        Self { data: value }
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data + rhs.data,
        }
    }
}

impl AddAssign<Vector4> for Vector4 {
    fn add_assign(&mut self, rhs: Vector4) {
        self.data += rhs.data
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data / rhs.data,
        }
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data * rhs.data,
        }
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data - rhs.data,
        }
    }
}
