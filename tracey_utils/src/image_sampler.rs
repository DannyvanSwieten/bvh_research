use libraytracer::types::{HdrColor, Vec2};

use crate::texture::ImageTexture;

pub trait ImageSampler {
    fn sample_1d(&self, image: &ImageTexture, location: Vec2) -> HdrColor;
    fn sample_2d(&self, image: &ImageTexture, location: Vec2) -> HdrColor;
    fn sample_3d(&self, image: &ImageTexture, location: Vec2) -> HdrColor;
}

pub struct NearestNeighborSampler {}

impl ImageSampler for NearestNeighborSampler {
    fn sample_1d(&self, image: &ImageTexture, location: Vec2) -> HdrColor {
        let x = (location.x * image.image.width() as f32) as u32;
        let pixel = image.image.get_pixel(x, 0);
        HdrColor::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }

    fn sample_2d(&self, image: &ImageTexture, location: Vec2) -> HdrColor {
        let x = (location.x * image.width() as f32) as u32;
        let y = (location.y * image.height() as f32) as u32;
        let pixel = image.image.get_pixel(x, y);
        HdrColor::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }

    fn sample_3d(&self, image: &ImageTexture, location: Vec2) -> HdrColor {
        let x = (location.x * image.width() as f32) as u32;
        let y = (location.y * image.height() as f32) as u32;
        let pixel = image.image.get_pixel(x, y);
        HdrColor::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }
}
