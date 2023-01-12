#![feature(portable_simd)]

pub mod acc_bvh_midpoint_split;
pub mod acceleration_structure;
pub mod brute_force;
pub mod bvh;
pub mod camera;
pub mod frame_buffer;
pub mod intersect;
pub mod trace;
pub mod types;

use std::{io::BufRead, time::Instant};

use image::ColorType;
use rand::Rng;
use types::{vec3, Triangle, Vertex};

use crate::{
    acc_bvh_midpoint_split::AccMidPointSplit,
    acceleration_structure::AccelerationStructure,
    brute_force::BruteForceStructure,
    camera::Camera,
    frame_buffer::Framebuffer,
    trace::{CpuTracer, Tracer},
    types::{HdrColor, Position},
};

fn read_triangle_file(name: &str) -> (Vec<Vertex>, Vec<Triangle>) {
    let path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/assets/"
        + name;
    let file = std::fs::File::open(path).expect("Couldn't open file");
    let reader = std::io::BufReader::new(file);
    let positions = reader
        .lines()
        .into_iter()
        .flat_map(|line| match line {
            Ok(line) => {
                let floats = line
                    .split(' ')
                    .into_iter()
                    .map(|token| {
                        let v: f32 = token.parse().expect("float parse failed");
                        v
                    })
                    .collect::<Vec<f32>>();

                floats.into_iter()
            }

            Err(_) => todo!(),
        })
        .collect::<Vec<f32>>();

    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for i in (0..positions.len()).step_by(9) {
        vertices.push(vec3(positions[i], positions[i + 1], positions[i + 2]));
        vertices.push(vec3(positions[i + 3], positions[i + 4], positions[i + 5]));
        vertices.push(vec3(positions[i + 6], positions[i + 7], positions[i + 8]));
    }

    for i in (0..vertices.len()).step_by(3) {
        triangles.push(Triangle {
            v0: i as u32,
            v1: i as u32 + 1,
            v2: i as u32 + 2,
        });
    }

    (vertices, triangles)
}

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

        let v0 = vec3(x0, y0, z0) * 9. - vec3(5., 5., 5.);
        let v1 = v0 + vec3(x1, y1, z1);
        let v2 = v0 + vec3(x2, y2, z2);

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

pub fn write_to_file(name: &str, framebuffer: &Framebuffer<HdrColor>) {
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
        name,
        &pixels,
        framebuffer.width() as _,
        framebuffer.height() as _,
        ColorType::Rgba8,
    )
    .expect("Image write failed");
}

fn main() {
    let (vertices, triangles) = read_triangle_file("unity.tri");
    let mut framebuffer = Framebuffer::new(640, 640, HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let camera = Camera::new(Position::new(-4.5, -0.2, -5.5), 2.0);
    let tracer = CpuTracer {};

    let mut brute_force_acc = BruteForceStructure::new();
    brute_force_acc.build(&vertices, &triangles);

    let mut midpoint_split_acc = AccMidPointSplit::new(true);
    midpoint_split_acc.build(&vertices, &triangles);

    let now = Instant::now();
    tracer.trace(&camera, &mut framebuffer, &brute_force_acc);
    let elapsed_time = now.elapsed();
    println!("Tracing took {} millis.", elapsed_time.as_millis());
    write_to_file("brute_force.png", &framebuffer);

    framebuffer.clear(HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let now = Instant::now();
    tracer.trace(&camera, &mut framebuffer, &midpoint_split_acc);
    let elapsed_time = now.elapsed();
    println!("Tracing took {} millis.", elapsed_time.as_millis());
    write_to_file("midpoint.png", &framebuffer);
}
