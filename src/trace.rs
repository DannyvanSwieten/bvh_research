use crossbeam::channel::unbounded;
use rayon::prelude::*;

use crate::{
    cpu::cpu_shader_binding_table::ShaderBindingTable,
    top_level_acceleration_structure::TopLevelAccelerationStructure, types::Vec2,
};

pub trait Tracer<Context, Payload> {
    fn trace(
        &self,
        ctx: &Context,
        width: u32,
        height: u32,
        shader_binding_table: &ShaderBindingTable<Context, Payload>,
        acceleration_structure: &TopLevelAccelerationStructure,
        result_buffer: &mut [Payload],
    );
}

pub struct CpuTracer {}
impl<Context: Send + Sync, Payload: Send + Sync + Default + Clone> Tracer<Context, Payload>
    for CpuTracer
{
    fn trace(
        &self,
        ctx: &Context,
        width: u32,
        height: u32,
        shader_binding_table: &ShaderBindingTable<Context, Payload>,
        acceleration_structure: &TopLevelAccelerationStructure,
        result_buffer: &mut [Payload],
    ) {
        let (tx, rx) = unbounded();
        (0..height).into_par_iter().for_each_with(tx, |tx, y| {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                let mut payload = Payload::default();
                shader_binding_table.ray_generation_shader().execute(
                    ctx,
                    acceleration_structure,
                    shader_binding_table,
                    &mut payload,
                    &Vec2::new(x as f32, y as f32),
                    &Vec2::new(width as f32, height as f32),
                );
                row.push(payload);
            }

            tx.send((y, row)).unwrap();
        });

        for (row, data) in rx {
            let y_offset = (row * width) as usize;
            for (index, pixel) in data.into_iter().enumerate() {
                result_buffer[y_offset + index] = pixel;
            }
        }
    }
}
