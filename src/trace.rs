use crossbeam::channel::unbounded;
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

        let (tx, rx) = unbounded();

        (0..height).into_par_iter().for_each_with(tx, |tx, y| {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let ray = camera.ray(x, y, width, height);

                let record = acceleration_structure.traverse(&ray);
                row.push(record);
            }
            tx.send((y, row)).expect("send failed");
        });

        for (row, data) in rx {
            data.iter().enumerate().for_each(|(x, record)| {
                if record.t < f32::MAX {
                    framebuffer.set_pixel(
                        x,
                        row,
                        HdrColor::new(1.0 - record.u - record.v, record.u, record.v, 1.0),
                    )
                }
            });
        }
    }
}
