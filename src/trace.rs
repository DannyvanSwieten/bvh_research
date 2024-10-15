use rayon::prelude::*;

use crate::{
    camera::Camera, frame_buffer::Framebuffer,
    top_level_acceleration_structure::TopLevelAccelerationStructure, types::HdrColor,
};

pub trait Tracer {
    fn trace(
        &self,
        camera: &Camera,
        framebuffer: &mut Framebuffer<HdrColor>,
        acceleration_structure: &TopLevelAccelerationStructure,
    );
}

pub struct CpuTracer {}
impl Tracer for CpuTracer {
    fn trace(
        &self,
        camera: &Camera,
        framebuffer: &mut Framebuffer<HdrColor>,
        acceleration_structure: &TopLevelAccelerationStructure,
    ) {
        let width = framebuffer.width();
        let height = framebuffer.height();

        let pixels = framebuffer.pixels_mut();
        let bands: Vec<(usize, &mut [cgmath::Vector4<f32>])> =
            pixels.chunks_mut(width).enumerate().collect();

        bands.into_par_iter().for_each(|(y, row)| {
            (0..width).for_each(|x| {
                let ray = camera.ray(x, y, width, height);

                let record = acceleration_structure.traverse(&ray);
                let pixel = &mut row[x];
                if record.t < f32::MAX {
                    *pixel =
                        cgmath::Vector4::new(1.0 - record.u - record.v, record.u, record.v, 1.0);
                }
            });
        });
    }
}
