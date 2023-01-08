use crate::{
    acceleration_structure::AccelerationStructure, camera::Camera, frame_buffer::Framebuffer,
    types::HdrColor,
};

pub trait Tracer {
    fn trace(
        &self,
        camera: &Camera,
        framebuffer: &mut Framebuffer<HdrColor>,
        acceleration_structure: &impl AccelerationStructure,
    );
}

pub struct CpuTracer {}
impl Tracer for CpuTracer {
    fn trace(
        &self,
        camera: &Camera,
        framebuffer: &mut Framebuffer<HdrColor>,
        acceleration_structure: &impl AccelerationStructure,
    ) {
        let width = framebuffer.width();
        let height = framebuffer.height();
        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray(x, y, width, height);
                let (_, t) = acceleration_structure.trace(&ray);
                if t < f32::MAX {
                    framebuffer.set_pixel(x, y, HdrColor::new(1.0, 1.0, 1.0, 1.0))
                }
            }
        }
    }
}
