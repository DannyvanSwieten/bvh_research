use rayon::prelude::*;
use std::sync::mpsc;

use crate::{
    camera::Camera,
    frame_buffer::Framebuffer,
    top_level_acceleration_structure::TopLevelAccelerationStructure,
    types::{HdrColor, Ray},
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
        // for y in 0..height {
        //     // let (tx, rx) = mpsc::channel::<Vec<HdrColor>>();
        //     // let handle = std::thread::spawn(move || {
        //     for x in 0..width {
        //         let ray = camera.ray(x, y, width, height);
        //         let record = acceleration_structure.traverse(&ray);
        //         if record.t < f32::MAX {
        //             framebuffer.set_pixel(
        //                 x,
        //                 y,
        //                 HdrColor::new(1.0 - record.u - record.v, record.u, record.v, 1.0),
        //             );
        //         }
        //     }
        // }

        (0..height).into_par_iter().for_each(|y| {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let ray = camera.ray(x, y, width, height);
                let record = acceleration_structure.traverse(&ray);
                if record.t < f32::MAX {
                    // framebuffer.set_pixel(
                    //     x,
                    //     y,
                    //     HdrColor::new(1.0 - record.u - record.v, record.u, record.v, 1.0),
                    // );

                    row.push(HdrColor::new(
                        1.0 - record.u - record.v,
                        record.u,
                        record.v,
                        1.0,
                    ));
                }
            }
        });
    }
    // }
    // }
    // });

    // handle.join().unwrap();

    // }
}
