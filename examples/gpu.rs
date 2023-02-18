use std::{mem::size_of, rc::Rc, time::Instant};

use cgmath::Vector3;
use gpu_tracer::{
    blas::{Blas, Instance},
    gpu_acceleration_structure::GpuTlas,
    gpu_ray_accumulator::GpuRayAccumulator,
    gpu_ray_generator::GpuRayGenerator,
    gpu_ray_intersector::GpuIntersector,
    gpu_ray_shader::GpuRayShader,
    read_triangle_file,
    types::{HdrColor, Mat4, Vertex},
    write_hdr_buffer_to_file, write_intersection_buffer_to_file, write_ray_buffer_to_file,
};
use image::Frame;
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    image2d_resource::Image2DResource, queue::CommandQueue, vulkan::Vulkan, BufferUsageFlags,
    DebugUtils, Format, ImageUsageFlags, MemoryPropertyFlags, PhysicalDeviceFeatures2KHR,
    PhysicalDeviceVulkan12Features, QueueFlags,
};

fn load_shader(name: &str) -> String {
    let path = std::env::current_dir()
        .unwrap()
        .join("example_shaders")
        .join(name);

    std::fs::read_to_string(path).expect("Reading Shader File Failed")
}
#[derive(Clone, Copy)]
struct FrameData {
    pub frame: u32,
    pub bounce: u32,
}

fn main() {
    let vulkan = Vulkan::new(
        "My Application",
        &["VK_LAYER_KHRONOS_validation"],
        &[DebugUtils::name().to_str().unwrap()],
    );

    let physical_devices = vulkan.physical_devices();
    let graphics_compute_index = physical_devices
        .iter()
        .position(|device| device.supports_compute());

    let logical_device = if let Some(index) = graphics_compute_index {
        let gpu = &physical_devices[index];
        let mut address_features = PhysicalDeviceVulkan12Features::builder()
            .buffer_device_address(true)
            .shader_input_attachment_array_dynamic_indexing(true)
            .descriptor_indexing(true)
            .runtime_descriptor_array(true)
            .build();
        let mut features2 = PhysicalDeviceFeatures2KHR::default();
        unsafe {
            gpu.vulkan()
                .vk_instance()
                .get_physical_device_features2(*gpu.vk_physical_device(), &mut features2);
        }
        gpu.device_context_builder(&["VK_KHR_buffer_device_address"], |builder| {
            builder
                .push_next(&mut address_features)
                .enabled_features(&features2.features)
        })
    } else {
        panic!()
    };

    let logical_device = Rc::new(logical_device);
    let (vertices, indices) = read_triangle_file("unity.tri");
    let vertex_buffer = BufferResource::new_host_visible_storage(
        logical_device.clone(),
        size_of::<Vertex>() * vertices.len(),
    )
    .with_data(&vertices);
    let index_buffer = BufferResource::new_host_visible_storage(
        logical_device.clone(),
        size_of::<u32>() * vertices.len(),
    )
    .with_data(&indices);
    let blas = Rc::new(Blas::new(
        logical_device.clone(),
        &vertex_buffer,
        &index_buffer,
    ));
    let gpu_instances = [
        Instance::new(blas.clone(), 0).with_transform(Mat4::from_scale(0.25)),
        // Instance::new(blas, 1)
        //     .with_transform(Mat4::from_translation(Vector3::<f32>::new(0.0, 1.0, 0.0))),
    ];

    let debug = true;

    let acceleration_structure = GpuTlas::new(logical_device.clone(), &gpu_instances);

    let queue = Rc::new(CommandQueue::new(
        logical_device.clone(),
        QueueFlags::COMPUTE,
    ));
    let width = 720;
    let height = 640;
    let ray_generation_shader = load_shader("ray_gen.glsl");
    let mut gpu_ray_generator =
        GpuRayGenerator::new_from_string(logical_device.clone(), &ray_generation_shader, 1, None);
    let mut gpu_intersector = GpuIntersector::new(logical_device.clone(), 1);
    let ray_buffer = gpu_ray_generator.allocate_ray_buffer(width, height, false);
    let intersection_buffer = gpu_intersector.allocate_intersection_buffer(width, height, false);

    let ray_shader = load_shader("ray_shader.glsl");
    let mut gpu_shader =
        GpuRayShader::new_from_string(logical_device.clone(), &ray_shader, 1, None);
    let mut ray_accumulator = GpuRayAccumulator::new(logical_device.clone(), 1);
    let mut image = Image2DResource::new(
        logical_device.clone(),
        width as _,
        height as _,
        Format::R32G32B32A32_SFLOAT,
        ImageUsageFlags::STORAGE | ImageUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::DEVICE_LOCAL,
    );

    let mut frame_data = FrameData {
        frame: 0,
        bounce: 0,
    };
    let now = Instant::now();
    for sample in 0..16 {
        let mut command_buffer = CommandBuffer::new(queue.clone());
        command_buffer.begin();
        gpu_ray_generator.generate_rays(
            &mut command_buffer,
            width,
            height,
            &ray_buffer,
            &frame_data,
        );

        gpu_intersector.intersect(
            &mut command_buffer,
            width,
            height,
            &ray_buffer,
            &intersection_buffer,
            &acceleration_structure,
        );
        gpu_shader.set_buffer(1, 0, &index_buffer);
        gpu_shader.set_buffer(1, 1, &vertex_buffer);
        gpu_shader.shade_rays(
            &mut command_buffer,
            width,
            height,
            &ray_buffer,
            &intersection_buffer,
            &acceleration_structure,
        );

        ray_accumulator.accumulate(&mut command_buffer, &ray_buffer, &mut image);
        command_buffer.submit();
        frame_data.frame += 1
    }
    let elapsed_time = now.elapsed();
    println!("Tracing GPU took {} millis.", elapsed_time.as_millis());
    if debug {
        let mut staging_buffer = BufferResource::new(
            logical_device.clone(),
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
            (frame_data.frame as usize).max(1),
            &pixels,
            width,
            height,
        );
    }
}
