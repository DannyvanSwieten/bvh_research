use std::{path::PathBuf, rc::Rc, time::Instant};

use intersect::{
    gpu::{
        blas::Geometry,
        gpu::Gpu,
        gpu_acceleration_structure::GpuTlas,
        instance::Instance,
        ray_tracing_pipeline::RayTracingPipeline,
        ray_tracing_pipeline_descriptor::{
            BufferDescriptor, ImageDescriptor, PayloadDescriptor, RayTracingPipelineDescriptor,
            ShaderFunction, ShaderSource, StructAttribute,
        },
    },
    read_triangle_file,
    types::{DataType, HdrColor, Mat4},
    write_hdr_buffer_to_file,
};
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    image2d_resource::Image2DResource, queue::CommandQueue, Format, ImageLayout, QueueFlags,
};

fn shader_path() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("intersect/example_shaders")
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
        // Instance::new(procedural_blas.clone(), 1),
    ];

    let acceleration_structure = GpuTlas::new(device_context.clone(), &gpu_instances);

    let shader_path = shader_path();
    let ray_generator_source = ShaderSource::File(shader_path.join("ray_gen.glsl"));
    let ray_shader_source = ShaderSource::File(shader_path.join("closest_hit.glsl"));
    let ray_miss_source = ShaderFunction::new(
        ShaderSource::File(shader_path.join("miss.glsl")),
        "first_miss_shader",
    );

    let pipeline_descriptor =
        RayTracingPipelineDescriptor::new(ray_generator_source, ray_shader_source)
            .with_miss_function(ray_miss_source)
            .with_ray_payload_descriptor(PayloadDescriptor::new().with_attribute(
                "color",
                StructAttribute {
                    data_type: DataType::Vec3,
                    ..Default::default()
                },
            ))
            .with_buffer_descriptor(
                BufferDescriptor::new("IndexBuffer")
                    .with_attribute(
                        "indices",
                        StructAttribute {
                            data_type: DataType::Uint32,
                            is_array: true,
                            ..Default::default()
                        },
                    )
                    .with_read_only(true),
            )
            .with_buffer_descriptor(
                BufferDescriptor::new("VertexBuffer")
                    .with_attribute(
                        "vertices",
                        StructAttribute {
                            data_type: DataType::Float,
                            is_array: true,
                            ..Default::default()
                        },
                    )
                    .with_read_only(true),
            )
            .with_image_descriptor(
                ImageDescriptor::new("result")
                    .with_float(true)
                    .with_bits_per_channel(32)
                    .with_read_only(false),
            );
    let mut pipeline = RayTracingPipeline::new(device_context.clone(), &pipeline_descriptor);

    let width = 512;
    let height = 512;

    let mut image = Image2DResource::new_device_local_storage_image(
        device_context.clone(),
        width,
        height,
        Format::R32G32B32A32_SFLOAT,
    );

    let queue = Rc::new(CommandQueue::new(
        device_context.clone(),
        QueueFlags::COMPUTE,
    ));
    let mut transition_command_buffer = CommandBuffer::new(queue.clone());
    transition_command_buffer.begin();
    transition_command_buffer.image_resource_transition(&mut image, ImageLayout::GENERAL);
    transition_command_buffer.submit();

    let mut command_buffer = CommandBuffer::new(queue.clone());

    // let frame_data = pipeline.prepare_to_render(width, height);
    let progress = Progress {
        frame: 0,
        bounce: 0,
    };
    let now = Instant::now();
    pipeline.set_storage_image(0, &image);
    pipeline.set_storage_buffer(0, &index_buffer);
    pipeline.set_storage_buffer(1, &vertex_buffer);
    command_buffer.begin();
    pipeline.trace(
        width,
        height,
        &acceleration_structure,
        Some(&progress),
        &mut command_buffer,
    );
    command_buffer.submit();
    println!("Elapsed: {:?} milliseconds", now.elapsed().as_millis());

    let mut transfer_buffer = BufferResource::new_host_visible_storage(
        device_context.clone(),
        size_of::<HdrColor>() * width as usize * height as usize,
    );

    let mut transfer_command_buffer = CommandBuffer::new(queue.clone());
    transfer_command_buffer.begin();
    transfer_command_buffer.copy_image_to_buffer(&image, &mut transfer_buffer);
    transfer_command_buffer.submit();

    write_hdr_buffer_to_file(
        "result.png",
        1,
        &transfer_buffer.copy_data::<HdrColor>(),
        width as _,
        height as _,
    );

    // let ray_buffer_data: Vec<Ray> = frame_data.ray_buffer.copy_data();
    // write_ray_buffer_to_file(
    //     "ray_buffer.png",
    //     &ray_buffer_data,
    //     width as usize,
    //     height as usize,
    // );
}
