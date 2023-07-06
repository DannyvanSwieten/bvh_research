use std::rc::Rc;

use libraytracer::{
    cpu::shape::SurfaceAttributes,
    types::{Direction, HdrColor},
};

use crate::{onb::OrthoNormalBasis, sampler::Sampler, texture::Texture};

pub trait Material {
    fn scatter(&self, sampler: &dyn Sampler, attributes: &SurfaceAttributes) -> Direction;
    fn bsdf(&self, attributes: &SurfaceAttributes, direction: &Direction) -> HdrColor;
    fn albedo(&self, attributes: &SurfaceAttributes) -> HdrColor;
    fn emission(&self, _attributes: &SurfaceAttributes) -> HdrColor {
        HdrColor::new(0.0, 0.0, 0.0, 1.0)
    }
}

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, sampler: &dyn Sampler, attributes: &SurfaceAttributes) -> Direction {
        let onb = OrthoNormalBasis::new(&attributes.normal);
        onb.to_local(&sampler.sample_hemisphere())
    }

    fn bsdf(&self, attributes: &SurfaceAttributes, _direction: &Direction) -> HdrColor {
        self.albedo.sample(attributes)
    }

    fn albedo(&self, attributes: &SurfaceAttributes) -> HdrColor {
        self.albedo.sample(attributes)
    }
}

pub struct Mirror {
    pub albedo: Rc<dyn Texture>,
}

impl Mirror {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Mirror {
    fn scatter(&self, _sampler: &dyn Sampler, _attributes: &SurfaceAttributes) -> Direction {
        //reflect(&Direction::new(1.0, 1.0, 1.0), &attributes.normal)
        Direction::new(1.0, 1.0, 1.0)
    }

    fn bsdf(&self, attributes: &SurfaceAttributes, _direction: &Direction) -> HdrColor {
        self.albedo.sample(attributes)
    }

    fn albedo(&self, attributes: &SurfaceAttributes) -> HdrColor {
        self.albedo.sample(attributes)
    }
}

pub struct Emissive {
    pub albedo: Rc<dyn Texture>,
}

impl Emissive {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Emissive {
    fn scatter(&self, sampler: &dyn Sampler, attributes: &SurfaceAttributes) -> Direction {
        let onb = OrthoNormalBasis::new(&attributes.normal);
        onb.to_local(&sampler.sample_hemisphere())
    }

    fn bsdf(&self, _attributes: &SurfaceAttributes, _direction: &Direction) -> HdrColor {
        HdrColor::new(0.0, 0.0, 0.0, 1.0)
    }

    fn emission(&self, attributes: &SurfaceAttributes) -> HdrColor {
        self.albedo.sample(attributes)
    }

    fn albedo(&self, attributes: &SurfaceAttributes) -> HdrColor {
        self.albedo.sample(attributes)
    }
}
