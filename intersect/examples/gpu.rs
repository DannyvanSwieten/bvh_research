use std::{rc::Rc, time::Instant};

use intersect::{
    gpu::{
        blas::{Blas, Instance},
        gpu::Gpu,
        gpu_acceleration_structure::GpuTlas,
        gpu_ray_accumulator::GpuRayAccumulator,
        gpu_ray_generator::GpuRayGenerator,
        gpu_ray_intersector::GpuIntersector,
        gpu_ray_shader::GpuRayShader,
    },
    read_triangle_file,
    types::{HdrColor, Mat4, Ray, Vec3},
    write_hdr_buffer_to_file, write_ray_buffer_to_file,
};

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    image2d_resource::Image2DResource, queue::CommandQueue, AccessFlags, BufferUsageFlags, Format,
    ImageLayout, ImageUsageFlags, MemoryPropertyFlags, PipelineStageFlags, QueueFlags,
};

fn load_shader(name: &str) -> String {
    let path = std::env::current_dir()
        .unwrap()
        .join("example_shaders")
        .join(name);

    std::fs::read_to_string(path).expect("Reading Shader File Failed")
}
#[derive(Clone, Copy)]
struct Progress {
    pub frame: u32,
    pub bounce: u32,
}

fn main() {
    let gpu = Gpu::new("My Application");
    let device_context = Rc::new(gpu.create_device(0));
    let (vertices, indices) = read_triangle_file("unity.tri");
    let vertex_buffer =
        BufferResource::new_host_visible_with_data(device_context.clone(), &vertices);
    let index_buffer = BufferResource::new_host_visible_with_data(device_context.clone(), &indices);
    let blas = Rc::new(Blas::new(
        device_context.clone(),
        &vertex_buffer,
        &index_buffer,
    ));
    let gpu_instances = [
        Instance::new(blas.clone(), 0).with_transform(Mat4::from_scale(0.25)),
        // Instance::new(blas.clone(), 1).with_transform(
        //     Mat4::from_translation(Vec3::new(0.5, 0.5, 2.0)) * Mat4::from_scale(0.25),
        // ),
    ];

    let debug = true;

    let acceleration_structure = GpuTlas::new(device_context.clone(), &gpu_instances);

    let queue = Rc::new(CommandQueue::new(
        device_context.clone(),
        QueueFlags::COMPUTE,
    ));
    let width = 1000;
    let height = 1000;
    let frame_data = gpu.create_frame_data(device_context.clone(), width, height);
    let ray_generation_shader = load_shader("ray_gen.glsl");
    let mut gpu_ray_generator =
        GpuRayGenerator::new_from_string(device_context.clone(), &ray_generation_shader, 1, None);
    let mut gpu_intersector = GpuIntersector::new(device_context.clone(), 1);
    let ray_buffer = gpu_ray_generator.allocate_ray_buffer(&frame_data, false);
    let intersection_buffer = gpu_intersector.allocate_intersection_buffer(width, height, false);

    let ray_shader = load_shader("ray_shader.glsl");
    let mut gpu_shader =
        GpuRayShader::new_from_string(device_context.clone(), &ray_shader, 1, None);
    let mut ray_accumulator = GpuRayAccumulator::new(device_context.clone(), 1);
    let mut image = Image2DResource::new(
        device_context.clone(),
        width as _,
        height as _,
        Format::R32G32B32A32_SFLOAT,
        ImageUsageFlags::STORAGE | ImageUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::DEVICE_LOCAL,
    );

    let mut transition_buffer = CommandBuffer::new(queue.clone());
    transition_buffer.begin();
    transition_buffer.image_resource_transition(&mut image, ImageLayout::GENERAL);
    transition_buffer.submit();

    let mut progress = Progress {
        frame: 0,
        bounce: 0,
    };
    gpu_ray_generator.set_ray_buffer(&ray_buffer);
    gpu_intersector.set(&ray_buffer, &intersection_buffer, &acceleration_structure);
    gpu_shader.set_user_buffer(1, 0, &index_buffer);
    gpu_shader.set_user_buffer(1, 1, &vertex_buffer);
    gpu_shader.set(&ray_buffer, &intersection_buffer, &acceleration_structure);
    ray_accumulator.set(&ray_buffer, &image);
    let now = Instant::now();
    let mut command_buffer = CommandBuffer::new(queue.clone());
    command_buffer.begin();
    for _ in 0..1 {
        gpu_ray_generator.generate_rays(&mut command_buffer, &frame_data, Some(&progress));

        command_buffer.buffer_resource_barrier(
            &ray_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );

        gpu_intersector.intersect(&mut command_buffer, &frame_data);
        command_buffer.buffer_resource_barrier(
            &intersection_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );
        command_buffer.buffer_resource_barrier(
            &ray_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );

        gpu_shader.shade_rays(&mut command_buffer, &frame_data);
        command_buffer.buffer_resource_barrier(
            &ray_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );

        ray_accumulator.accumulate(&frame_data, &mut command_buffer);
        command_buffer.image_resource_transition(&mut image, ImageLayout::GENERAL);
        progress.frame += 1
    }
    command_buffer.submit();
    let elapsed_time = now.elapsed();
    println!("Tracing GPU took {} millis.", elapsed_time.as_millis());
    if debug {
        let mut staging_buffer = BufferResource::new(
            device_context.clone(),
            width * height * 16,
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::TRANSFER_DST,
        );

        let mut command_buffer = CommandBuffer::new(queue.clone());
        command_buffer.begin();
        command_buffer.copy_image_to_buffer(&image, &mut staging_buffer);
        command_buffer.submit();
        let pixels: Vec<HdrColor> = staging_buffer.copy_data();
        write_hdr_buffer_to_file(
            "accumulation_buffer.png",
            (progress.frame as usize).max(1),
            &pixels,
            width,
            height,
        );

        // let ray_buffer_data: Vec<Ray> = ray_buffer.copy_data();
        // write_ray_buffer_to_file("ray_buffer.png", &ray_buffer_data, width, height);
    }
}
