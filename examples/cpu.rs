use std::{rc::Rc, time::Instant};

use gpu_tracer::{
    bvh::Bvh,
    camera::Camera,
    frame_buffer::Framebuffer,
    read_triangle_file,
    top_level_acceleration_structure::{Instance, TopLevelAccelerationStructure},
    trace::{CpuTracer, Tracer},
    types::{HdrColor, Mat4, Position},
    write_framebuffer_to_file,
};

fn main() {
    let (vertices, indices) = read_triangle_file("unity.tri");
    let mut framebuffer = Framebuffer::new(640, 640, HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let camera = Camera::new(Position::new(-5.0, 0.0, -15.0), 2.0);
    let tracer = CpuTracer {};

    let midpoint_split_acc = Rc::new(Bvh::new(&vertices, &indices));

    let instances = [Instance::new(midpoint_split_acc, 0, Mat4::from_scale(1.0))];

    let tlas = TopLevelAccelerationStructure::new(&instances);

    framebuffer.clear(HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let now = Instant::now();
    tracer.trace(&camera, &mut framebuffer, &tlas);
    let elapsed_time = now.elapsed();
    println!("Tracing CPU took {} millis.", elapsed_time.as_millis());
    write_framebuffer_to_file("cpu.png", &framebuffer);
}
