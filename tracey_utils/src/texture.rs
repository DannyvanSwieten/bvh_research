use std::rc::Rc;

use libraytracer::{cpu::shape::SurfaceAttributes, types::HdrColor};

pub trait Texture {
    fn sample(&self, attributes: &SurfaceAttributes) -> HdrColor;
}

pub struct UniformColorTexture {
    pub color: HdrColor,
}

impl UniformColorTexture {
    pub fn new(color: HdrColor) -> Self {
        Self { color }
    }
}

impl Texture for UniformColorTexture {
    fn sample(&self, _attributes: &SurfaceAttributes) -> HdrColor {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
    pub scale: f32,
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>, scale: f32) -> Self {
        Self { odd, even, scale }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, attributes: &SurfaceAttributes) -> HdrColor {
        let sines =
            (self.scale * attributes.position.x).sin() * (self.scale * attributes.position.y).sin();
        if sines < 0.0 {
            self.odd.sample(attributes)
        } else {
            self.even.sample(attributes)
        }
    }
}

pub struct NoiseTexture<NoiseGenerator: noise::NoiseFn<f64, 3>> {
    pub scale: f64,
    pub noise_generator: NoiseGenerator,
}

impl<NoiseGenerator: noise::NoiseFn<f64, 3>> NoiseTexture<NoiseGenerator> {
    pub fn new(scale: f64, noise_generator: NoiseGenerator) -> Self {
        Self {
            scale,
            noise_generator,
        }
    }
}

impl<NoiseGenerator: noise::NoiseFn<f64, 3>> Texture for NoiseTexture<NoiseGenerator> {
    fn sample(&self, attributes: &SurfaceAttributes) -> HdrColor {
        let n = self.noise_generator.get([
            self.scale * attributes.position.x as f64,
            self.scale * attributes.position.y as f64,
            self.scale * attributes.position.z as f64,
        ]) as f32;

        HdrColor::new(n, n, n, 1.0)
    }
}

pub struct ImageTexture {
    pub image: image::Rgba32FImage,
}

impl ImageTexture {
    pub fn new(image: image::Rgba32FImage) -> Self {
        Self { image }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}
