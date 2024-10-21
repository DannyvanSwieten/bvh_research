use std::rc::Rc;

use cgmath::SquareMatrix;

use crate::types::{Mat4, AABB};

use super::blas::Geometry;

pub struct Instance {
    blas: Rc<Geometry>,
    id: u32,
    transform: Mat4,
}

impl Instance {
    pub fn new(blas: Rc<Geometry>, id: u32) -> Self {
        Self {
            blas,
            id,
            transform: Mat4::identity(),
        }
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }

    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }

    pub fn blas(&self) -> &Geometry {
        &self.blas
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn aabb(&self) -> &AABB {
        match self.blas.as_ref() {
            Geometry::Triangle(triangle) => triangle.aabb(),
            Geometry::Procedural(procedural) => procedural.aabb(),
        }
    }

    pub fn address(&self) -> u64 {
        match self.blas.as_ref() {
            Geometry::Triangle(triangle) => triangle.address(),
            Geometry::Procedural(_) => 0,
        }
    }
}
