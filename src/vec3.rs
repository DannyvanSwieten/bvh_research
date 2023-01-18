use std::{
    ops::{Add, AddAssign, Div, Index, Mul, Sub},
    simd::Simd,
};

#[derive(Clone, Copy)]
pub struct Vector3 {
    data: Simd<f32, 4>,
}

impl Vector3 {
    pub fn dot(&self, rhs: &Vector3) -> f32 {
        let f = self.data * rhs.data;
        f[0] + f[1] + f[2]
    }

    pub fn cross(&self, rhs: &Vector3) -> Self {
        let x = self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1];
        let y = self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2];
        let z = self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0];
        Self::from([x, y, z])
    }

    pub fn normalized(self) -> Self {
        let f = self * self;
        let d = f.dot(&f);
        self / d
    }
}

impl From<f32> for Vector3 {
    fn from(value: f32) -> Self {
        Self {
            data: Simd::splat(value),
        }
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            data: Simd::from([value[0], value[1], value[2], f32::default()]),
        }
    }
}

impl From<Simd<f32, 4>> for Vector3 {
    fn from(value: Simd<f32, 4>) -> Self {
        Self { data: value }
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data + rhs.data,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.data += rhs.data
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data / rhs.data,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            data: self.data / Simd::<f32, 4>::splat(rhs),
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data * rhs.data,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: self.data * Simd::<f32, 4>::splat(rhs),
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data - rhs.data,
        }
    }
}
