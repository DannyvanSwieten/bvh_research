use image::ColorType;
use libraytracer::{
    gpu::gpu_ray_intersector::IntersectionResult,
    types::{HdrColor, Ray},
};

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
            let r = (result.color.x.sqrt() * 255.0) as u8;
            let g = (result.color.y.sqrt() * 255.0) as u8;
            let b = (result.color.z.sqrt() * 255.0) as u8;
            let a = 255_u8;
            vec![r, g, b, a].into_iter()
        })
        .collect();
    image::save_buffer(name, &pixels, width as _, height as _, ColorType::Rgba8)
        .expect("Image write failed");
}

pub fn write_hdr_buffer_to_file(
    name: &str,
    sample_count: usize,
    buffer: &[HdrColor],
    width: usize,
    height: usize,
    to_ldr: bool,
    gamma_correction: bool,
) {
    let f = 1.0 / sample_count as f32;
    let pixels: Vec<u8> = buffer
        .iter()
        .flat_map(|result| {
            let mut pixel = result * f;
            if to_ldr {
                pixel.x = pixel.x / (1.0 + pixel.x);
                pixel.y = pixel.y / (1.0 + pixel.y);
                pixel.z = pixel.z / (1.0 + pixel.z);
            }

            if gamma_correction {
                pixel.x = pixel.x.sqrt();
                pixel.y = pixel.y.sqrt();
                pixel.z = pixel.z.sqrt();
            }

            let r = (pixel.x * 255.0) as u8;
            let g = (pixel.y * 255.0) as u8;
            let b = (pixel.z * 255.0) as u8;
            let a = 255_u8;
            vec![r, g, b, a].into_iter()
        })
        .collect();
    image::save_buffer(name, &pixels, width as _, height as _, ColorType::Rgba8)
        .expect("Image write failed");
}
