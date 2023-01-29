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
                let record = acceleration_structure.traverse(&ray);
                if record.t < f32::MAX {
                    framebuffer.set_pixel(
                        x,
                        y,
                        HdrColor::new(record.u, record.v, 1.0 - record.u - record.v, 1.0),
                    )
                }
            }
        }
    }
}
