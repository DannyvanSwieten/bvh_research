pub mod acc_bvh_midpoint_split;
pub mod acceleration_structure;
pub mod brute_force;
pub mod bvh;
pub mod camera;
pub mod frame_buffer;
pub mod intersect;
pub mod trace;
pub mod types;

use std::time::Instant;

use image::ColorType;
use rand::Rng;
use types::{Triangle, Vertex};

use crate::{
    acceleration_structure::AccelerationStructure,
    brute_force::BruteForceStructure,
    camera::Camera,
    frame_buffer::Framebuffer,
    trace::{CpuTracer, Tracer},
    types::{HdrColor, Position},
};

fn generate_random_vertices(count: usize) -> (Vec<Vertex>, Vec<Triangle>) {
    let mut rng = rand::thread_rng();
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for i in 0..count {
        let x0: f32 = rng.gen();
        let x1: f32 = rng.gen();
        let x2: f32 = rng.gen();

        let y0: f32 = rng.gen();
        let y1: f32 = rng.gen();
        let y2: f32 = rng.gen();

        let z0: f32 = rng.gen();
        let z1: f32 = rng.gen();
        let z2: f32 = rng.gen();

        let v0 = Vertex::new(x0, y0, z0) * 9. - Vertex::new(5., 5., 5.);
        let v1 = v0 + Vertex::new(x1, y1, z1);
        let v2 = v0 + Vertex::new(x2, y2, z2);

        vertices.push(v0);
        vertices.push(v1);
        vertices.push(v2);

        let idx = i as u32 * 3;
        let tri = Triangle {
            v0: idx,
            v1: idx + 1,
            v2: idx + 2,
        };

        triangles.push(tri);
    }

    (vertices, triangles)
}

pub fn write_to_file(framebuffer: &Framebuffer<HdrColor>) {
    let pixels: Vec<u8> = framebuffer
        .iter()
        .flat_map(|pixel| {
            let r = (pixel.x * 255.0) as u8;
            let g = (pixel.y * 255.0) as u8;
            let b = (pixel.z * 255.0) as u8;
            let a = (pixel.w * 255.0) as u8;
            vec![r, g, b, a].into_iter()
        })
        .collect();
    image::save_buffer(
        "output.png",
        &pixels,
        framebuffer.width() as _,
        framebuffer.height() as _,
        ColorType::Rgba8,
    )
    .expect("Image write failed");
}

fn main() {
    let (vertices, triangles) = generate_random_vertices(64);
    let mut framebuffer = Framebuffer::new(640, 640, HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let camera = Camera::new(Position::new(0.0, 0.0, -18.0), -15.0);
    let tracer = CpuTracer {};
    let mut acceleration_structure = BruteForceStructure::new();
    acceleration_structure.build(&vertices, &triangles);
    let now = Instant::now();
    tracer.trace(&camera, &mut framebuffer, &acceleration_structure);
    let elapsed_time = now.elapsed();
    println!("Tracing took {} millis.", elapsed_time.as_millis());
    write_to_file(&framebuffer);
}
