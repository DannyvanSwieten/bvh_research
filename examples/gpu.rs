use std::{mem::size_of, rc::Rc, time::Instant};

use gpu_tracer::{
    blas::{Blas, Instance},
    gpu_acceleration_structure::GpuTlas,
    gpu_ray_generator::GpuRayGenerator,
    gpu_ray_intersector::GpuIntersector,
    read_triangle_file,
    types::Vertex,
};
use vk_utils::{
    buffer_resource::BufferResource, queue::CommandQueue, vulkan::Vulkan, DebugUtils,
    PhysicalDeviceFeatures2KHR, PhysicalDeviceVulkan12Features, QueueFlags,
};

fn main() {
    let ray_generation_shader = r#"
    
    Ray create_ray(vec2 resolution, vec2 frag_location, vec3 origin, float z){
        vec2 norm = frag_location / resolution;
        vec3 p0 = vec3(-1, 1, z);
        vec3 p1 = vec3(1, 1, z);
        vec3 p2 = vec3(-1, -1, z);
    
        vec3 pixel_position = 
                p0
                + (p1 - p0) * norm.x
                + (p2 - p0) * norm.y;
    
        vec3 direction = normalize(pixel_position - origin);
        Ray ray;
        ray.origin = origin;
        ray.direction = direction;
        ray.inv_direction = 1.0 / direction;
        return ray;
    }

    Ray generate_ray(vec2 pixel, vec2 resolution){
        return create_ray(resolution, pixel, vec3(-5.0, 0.0, -15.0), 2.0);
    }
    
    "#;

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
        Instance::new(blas, 0),
        // Instance::new(blas.clone(), 1)
        //     .with_transform(Mat4::from_translation(Position::new(0.0, 1.0, 0.0))),
    ];

    let acceleration_structure = GpuTlas::new(logical_device.clone(), &gpu_instances);

    let queue = Rc::new(CommandQueue::new(logical_device, QueueFlags::COMPUTE));
    let width = 720;
    let height = 640;
    let mut gpu_ray_generator =
        GpuRayGenerator::new_from_string(queue.clone(), ray_generation_shader, 1, None);
    let mut gpu_intersector = GpuIntersector::new(queue, 1);
    let ray_buffer = gpu_ray_generator.allocate_ray_buffer(width, height, false);
    let intersection_buffer = gpu_intersector.allocate_intersection_buffer(width, height, false);

    let now = Instant::now();
    gpu_ray_generator.generate_rays(width, height, &ray_buffer);
    gpu_intersector
        .intersect(
            width,
            height,
            &ray_buffer,
            &intersection_buffer,
            &acceleration_structure,
        )
        .wait();
    let elapsed_time = now.elapsed();
    println!("Tracing GPU took {} millis.", elapsed_time.as_millis());
}
