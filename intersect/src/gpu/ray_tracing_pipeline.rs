use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    AccessFlags, BufferUsageFlags, MemoryPropertyFlags, PipelineStageFlags,
};

use crate::types::{Vec2, Vec3};

use super::{
    frame_data::FrameData,
    gpu_acceleration_structure::GpuTlas,
    gpu_ray_generator::GpuRayGenerator,
    gpu_ray_intersector::{GpuIntersector, IntersectionResult},
    gpu_ray_shader::GpuRayShader,
    ray_tracing_pipeline_descriptor::{RayTracingPipelineDescriptor, ShaderSource},
};

pub struct RayTracingPipeline {
    device: Rc<DeviceContext>,
    ray_generator: GpuRayGenerator,
    ray_intersector: GpuIntersector,
    ray_shader: GpuRayShader,
    ray_payload_size: usize,
    intersection_payload_size: usize,
}

impl RayTracingPipeline {
    pub fn new(device: Rc<DeviceContext>, descriptor: &RayTracingPipelineDescriptor) -> Self {
        let ray_struct = r"struct Ray {
            vec3 origin;
            vec3 direction;
        "
        .to_owned();

        let ray_struct = descriptor
            .ray_payload_descriptor()
            .attributes()
            .iter()
            .fold(ray_struct, |acc, (name, data_type)| {
                let type_str = format!("{} {};\n", data_type, name);
                acc + &type_str
            });

        let ray_struct = ray_struct + "};\n";

        let ray_generator = match &descriptor.ray_generation_source {
            ShaderSource::File(path) => GpuRayGenerator::new(
                device.clone(),
                path,
                &ray_struct,
                descriptor.max_frames_in_flight,
            ),
            ShaderSource::String(src) => GpuRayGenerator::new_from_string(
                device.clone(),
                src,
                &ray_struct,
                descriptor.max_frames_in_flight,
            ),
        };

        let intersection_struct = r"struct Intersection {
            float t;
            float u;
            float v;
            uint instance_id;
            uint primitive_id;
        "
        .to_owned();

        let intersection_struct = descriptor
            .intersection_payload_descriptor()
            .attributes()
            .iter()
            .fold(intersection_struct, |acc, (name, data_type)| {
                let type_str = format!("{} {};\n", data_type, name);
                acc + &type_str
            });

        let intersection_struct = intersection_struct + "};\n";

        let ray_shader = match &descriptor.ray_shader_source {
            ShaderSource::File(path) => GpuRayShader::new(
                device.clone(),
                path,
                &ray_struct,
                &intersection_struct,
                descriptor.max_frames_in_flight,
            ),
            ShaderSource::String(src) => GpuRayShader::new_from_string(
                device.clone(),
                src,
                &ray_struct,
                &intersection_struct,
                descriptor.max_frames_in_flight,
            ),
        };

        let ray_intersector = GpuIntersector::new(
            device.clone(),
            &ray_struct,
            &intersection_struct,
            &[],
            descriptor.max_frames_in_flight as usize,
        );

        let ray_payload_size =
            descriptor.ray_payload_descriptor().byte_size() + 2 * std::mem::size_of::<Vec3>();
        let intersection_payload_size = descriptor.intersection_payload_descriptor().byte_size()
            + std::mem::size_of::<IntersectionResult>();

        Self {
            device: device.clone(),
            ray_generator,
            ray_intersector,
            ray_shader,
            ray_payload_size,
            intersection_payload_size,
        }
    }

    pub fn prepare_to_render(&self, width: u32, height: u32) -> FrameData {
        let mut uniform_buffer = BufferResource::new(
            self.device.clone(),
            size_of::<Vec2>(),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::UNIFORM_BUFFER,
        );

        uniform_buffer.upload(&[width, height]);

        let ray_buffer = BufferResource::new(
            self.device.clone(),
            self.ray_payload_size * width as usize * height as usize,
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );

        let intersection_buffer = BufferResource::new(
            self.device.clone(),
            self.intersection_payload_size * width as usize * height as usize,
            MemoryPropertyFlags::DEVICE_LOCAL,
            BufferUsageFlags::STORAGE_BUFFER,
        );

        FrameData::new(
            width as _,
            height as _,
            uniform_buffer,
            ray_buffer,
            intersection_buffer,
        )
    }

    pub fn set_shader_buffer(&mut self, set: usize, binding: usize, buffer: &BufferResource) {
        self.ray_shader.set_user_buffer(set, binding, buffer);
    }

    pub fn trace<T: Copy>(
        &mut self,
        frame_data: &FrameData,
        acceleration_structure: &GpuTlas,
        constants: Option<&T>,
        command_buffer: &mut CommandBuffer,
    ) {
        self.ray_generator.set_ray_buffer(&frame_data.ray_buffer);
        self.ray_intersector.set(
            &frame_data.ray_buffer,
            &frame_data.intersection_buffer,
            acceleration_structure,
        );
        self.ray_shader.set(
            &frame_data.ray_buffer,
            &frame_data.intersection_buffer,
            acceleration_structure,
        );

        self.ray_generator
            .generate_rays(command_buffer, frame_data, constants);
        command_buffer.buffer_resource_barrier(
            &frame_data.ray_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );
        self.ray_intersector.intersect(command_buffer, frame_data);
        self.ray_shader.shade_rays(command_buffer, frame_data);
    }
}
