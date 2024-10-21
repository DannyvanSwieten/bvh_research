use std::{rc::Rc, time::Instant};

use intersect::{
    gpu::{
        blas::Geometry,
        gpu::Gpu,
        gpu_acceleration_structure::GpuTlas,
        instance::Instance,
        ray_tracing_pipeline::RayTracingPipeline,
        ray_tracing_pipeline_descriptor::{
            PayloadDescriptor, RayTracingPipelineDescriptor, ShaderSource,
        },
        triangle_blas::TriangleGeometry,
    },
    read_triangle_file,
    types::{DataType, HdrColor, Mat4, Ray, Vec3},
    write_hdr_buffer_to_file, write_ray_buffer_to_file,
};
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    image2d_resource::Image2DResource, queue::CommandQueue, QueueFlags,
};

fn load_shader(name: &str) -> String {
    let path = std::env::current_dir()
        .unwrap()
        .join("intersect/example_shaders")
        .join(name);

    println!("Reading shader: {}", path.display());
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
    let blas = Rc::new(Geometry::new_triangles(
        device_context.clone(),
        &vertex_buffer,
        &index_buffer,
    ));
    let gpu_instances = [
        Instance::new(blas.clone(), 0).with_transform(Mat4::from_scale(0.25)),
        Instance::new(blas.clone(), 1).with_transform(
            Mat4::from_translation(Vec3::new(0.5, 0.5, 2.0)) * Mat4::from_scale(0.25),
        ),
    ];

    let acceleration_structure = GpuTlas::new(device_context.clone(), &gpu_instances);

    let ray_generator_source = ShaderSource::String(load_shader("ray_gen.glsl"));
    let ray_shader_source = ShaderSource::String(load_shader("ray_shader.glsl"));

    let ray_descriptor = PayloadDescriptor::new().with_attribute("color", DataType::Vec4);

    let pipeline_descriptor =
        RayTracingPipelineDescriptor::new(ray_generator_source, ray_shader_source)
            .with_ray_payload_descriptor(ray_descriptor);
    let mut pipeline = RayTracingPipeline::new(device_context.clone(), &pipeline_descriptor);
    pipeline.set_shader_buffer(1, 0, &index_buffer);
    pipeline.set_shader_buffer(1, 1, &vertex_buffer);

    let width = 1000;
    let height = 1000;

    let queue = Rc::new(CommandQueue::new(
        device_context.clone(),
        QueueFlags::COMPUTE,
    ));
    let mut command_buffer = CommandBuffer::new(queue.clone());

    let frame_data = pipeline.prepare_to_render(width, height);
    let progress = Progress {
        frame: 0,
        bounce: 0,
    };
    command_buffer.begin();
    pipeline.trace(
        &frame_data,
        &acceleration_structure,
        Some(&progress),
        &mut command_buffer,
    );
    command_buffer.submit();

    let ray_buffer_data: Vec<Ray> = frame_data.ray_buffer.copy_data();
    write_ray_buffer_to_file(
        "ray_buffer.png",
        &ray_buffer_data,
        width as usize,
        height as usize,
    );
}
