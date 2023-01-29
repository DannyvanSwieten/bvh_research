use crate::{
    bottom_level_acceleration_structure::AccelerationStructure, camera::Camera,
    frame_buffer::Framebuffer, top_level_acceleration_structure::TopLevelAccelerationStructure,
    types::HdrColor,
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
        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray(x, y, width, height);
                let t = acceleration_structure.traverse(&ray);
                if t < f32::MAX {
                    let c = 1.0 / t;
                    framebuffer.set_pixel(x, y, HdrColor::new(c, c, c, 1.0))
                }
            }
        }
    }
}
