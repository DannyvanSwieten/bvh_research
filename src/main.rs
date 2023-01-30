pub mod bvh;
pub mod camera;
pub mod frame_buffer;
pub mod gpu_acceleration_structure;
pub mod intersect;
pub mod ray_generator;
pub mod top_level_acceleration_structure;
pub mod trace;
pub mod types;

use std::{io::BufRead, rc::Rc, time::Instant};

use cgmath::Matrix4;
use image::ColorType;
use types::{Triangle, Vertex};
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    image2d_resource::Image2DResource, pipeline_descriptor::ComputePipeline, queue::CommandQueue,
    vulkan::Vulkan, BufferUsageFlags, DebugUtils, Format, ImageLayout, ImageUsageFlags,
    MemoryPropertyFlags, QueueFlags,
};

use crate::{
    bvh::Bvh,
    camera::Camera,
    frame_buffer::Framebuffer,
    top_level_acceleration_structure::{Instance, TopLevelAccelerationStructure},
    trace::{CpuTracer, Tracer},
    types::{HdrColor, Position, Vec3},
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
        vertices.push(Vertex::new(
            positions[i],
            positions[i + 1],
            positions[i + 2],
            1.0,
        ));
        vertices.push(Vertex::new(
            positions[i + 3],
            positions[i + 4],
            positions[i + 5],
            1.0,
        ));
        vertices.push(Vertex::new(
            positions[i + 6],
            positions[i + 7],
            positions[i + 8],
            1.0,
        ));
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

fn main() {
    let (vertices, triangles) = read_triangle_file("unity.tri");
    let mut framebuffer = Framebuffer::new(640, 640, HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let camera = Camera::new(Position::new(-5.0, 0.0, -15.0), 2.0);
    let tracer = CpuTracer {};

    let midpoint_split_acc = Rc::new(Bvh::new(&vertices, &triangles, true));

    let instances = [
        Instance::new(
            midpoint_split_acc.clone(),
            0,
            Matrix4::<f32>::from_scale(1.0),
        ),
        Instance::new(
            midpoint_split_acc.clone(),
            2,
            Matrix4::<f32>::from_translation(Vec3::new(1.0, 1.0, 0.0)),
        ),
        Instance::new(
            midpoint_split_acc.clone(),
            2,
            Matrix4::<f32>::from_translation(Vec3::new(1.5, -1.0, 0.0)),
        ),
    ];

    let tlas = TopLevelAccelerationStructure::new(&instances);

    // let now = Instant::now();
    // tracer.trace(&camera, &mut framebuffer, &brute_force_acc);
    // let elapsed_time = now.elapsed();
    // println!(
    //     "Tracing brute force took {} millis.",
    //     elapsed_time.as_millis()
    // );
    // write_framebuffer_to_file("brute_force.png", &framebuffer);

    framebuffer.clear(HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let now = Instant::now();
    tracer.trace(&camera, &mut framebuffer, &tlas);
    let elapsed_time = now.elapsed();
    println!(
        "Tracing CPU SAH split took {} millis.",
        elapsed_time.as_millis()
    );
    write_framebuffer_to_file("midpoint.png", &framebuffer);

    let vulkan = Vulkan::new(
        "My Application",
        &[],
        &[DebugUtils::name().to_str().unwrap()],
    );

    let physical_devices = vulkan.physical_devices();
    let graphics_compute_index = physical_devices
        .iter()
        .position(|device| device.supports_compute());

    let logical_device = if let Some(index) = graphics_compute_index {
        physical_devices[index].device_context(&[])
    } else {
        panic!()
    };

    let shader_path = std::env::current_dir()
        .unwrap()
        .join("./assets/ray_intersector.comp");
    let logical_device = Rc::new(logical_device);
    let pipeline = ComputePipeline::new_from_source_file(
        shader_path.as_path(),
        logical_device.clone(),
        1,
        "main",
    );
    if let Some(mut pipeline) = pipeline {
        let mut bvh_buffer = BufferResource::new(
            logical_device.clone(),
            midpoint_split_acc.size(),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );
        bvh_buffer.upload(&midpoint_split_acc.nodes());
        pipeline.set_storage_buffer(0, 0, &bvh_buffer);

        let vertex_buffer_size = std::mem::size_of::<Vertex>() * vertices.len();
        let mut vertex_buffer = BufferResource::new(
            logical_device.clone(),
            vertex_buffer_size as _,
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );
        vertex_buffer.upload(&vertices);
        pipeline.set_storage_buffer(0, 1, &vertex_buffer);

        let index_buffer_size =
            std::mem::size_of::<Triangle>() * midpoint_split_acc.triangles().len();
        let mut index_buffer = BufferResource::new(
            logical_device.clone(),
            index_buffer_size as _,
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );
        index_buffer.upload(&midpoint_split_acc.triangles());
        pipeline.set_storage_buffer(0, 2, &index_buffer);

        let mut image = Image2DResource::new(
            logical_device.clone(),
            640,
            640,
            Format::R8G8B8A8_UNORM,
            ImageUsageFlags::STORAGE | ImageUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        let queue = Rc::new(CommandQueue::new(
            logical_device.clone(),
            QueueFlags::COMPUTE,
        ));
        let mut command_buffer = CommandBuffer::new(queue.clone());
        command_buffer.begin();
        command_buffer.image_resource_transition(&mut image, ImageLayout::GENERAL);
        pipeline.set_storage_image(0, 3, &image);
        command_buffer.bind_compute_pipeline(&pipeline);
        let now = Instant::now();
        command_buffer.dispatch_compute(640, 640, 1);
        command_buffer.submit().wait();
        let elapsed_time = now.elapsed();
        println!(
            "Tracing GPU SAH split took {} millis.",
            elapsed_time.as_millis()
        );

        let buffer_size = 4 * 640 * 640;
        let mut image_buffer = BufferResource::new(
            logical_device.clone(),
            buffer_size,
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::TRANSFER_DST,
        );
        let mut command_buffer = CommandBuffer::new(queue);
        command_buffer.begin();
        command_buffer.image_resource_transition(&mut image, ImageLayout::TRANSFER_SRC_OPTIMAL);
        command_buffer.copy_image_to_buffer(&image, &mut image_buffer);
        command_buffer.submit().wait();
        let pixel_buffer = image_buffer.copy_data::<u8>();
        image::save_buffer(
            "midpoint_gpu.png",
            &pixel_buffer,
            640,
            640,
            image::ColorType::Rgba8,
        )
        .expect("Image write failed");
    }
    print!("{}", 0);
}
