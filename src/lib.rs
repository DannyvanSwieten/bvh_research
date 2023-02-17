pub mod blas;
pub mod bvh;
pub mod camera;
pub mod cube;
pub mod frame_buffer;
pub mod gpu_acceleration_structure;
pub mod gpu_ray_generator;
pub mod gpu_ray_intersector;
pub mod gpu_ray_shader;
pub mod intersect;
pub mod top_level_acceleration_structure;
pub mod trace;
pub mod types;
pub mod vec3;

use std::io::BufRead;

use gpu_ray_intersector::IntersectionResult;
use image::ColorType;
use types::{Ray, Vertex};

use crate::{frame_buffer::Framebuffer, types::HdrColor};

pub fn read_triangle_file(name: &str) -> (Vec<Vertex>, Vec<u32>) {
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
        .flat_map(|line| match line {
            Ok(line) => {
                let floats = line
                    .split(' ')
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
        vertices.push(Vertex::new(
            positions[i],
            positions[i + 1],
            positions[i + 2],
        ));
        vertices.push(Vertex::new(
            positions[i + 3],
            positions[i + 4],
            positions[i + 5],
        ));
        vertices.push(Vertex::new(
            positions[i + 6],
            positions[i + 7],
            positions[i + 8],
        ));
    }

    for i in (0..vertices.len()).step_by(3) {
        triangles.push(i as u32);
        triangles.push(i as u32 + 1);
        triangles.push(i as u32 + 2);
    }

    (vertices, triangles)
}

pub fn write_framebuffer_to_file(name: &str, framebuffer: &Framebuffer<HdrColor>) {
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

pub fn write_intersection_buffer_to_file(
    name: &str,
    buffer: &[IntersectionResult],
    width: usize,
    height: usize,
) {
    let pixels: Vec<u8> = buffer
        .iter()
        .flat_map(|result| {
            if result.t < 10000.0 {
                let r = ((1.0 - result.u - result.v) * 255.0) as u8;
                let g = (result.u * 255.0) as u8;
                let b = (result.v * 255.0) as u8;
                let a = 255_u8;
                vec![r, g, b, a].into_iter()
            } else {
                vec![0, 0, 0, 0].into_iter()
            }
        })
        .collect();
    image::save_buffer(name, &pixels, width as _, height as _, ColorType::Rgba8)
        .expect("Image write failed");
}

pub fn write_ray_buffer_to_file(name: &str, buffer: &[Ray], width: usize, height: usize) {
    let pixels: Vec<u8> = buffer
        .iter()
        .flat_map(|result| {
            let r = (result.inv_direcion.x * 255.0) as u8;
            let g = (result.inv_direcion.y * 255.0) as u8;
            let b = (result.inv_direcion.z * 255.0) as u8;
            let a = 255_u8;
            vec![r, g, b, a].into_iter()
        })
        .collect();
    image::save_buffer(name, &pixels, width as _, height as _, ColorType::Rgba8)
        .expect("Image write failed");
}
