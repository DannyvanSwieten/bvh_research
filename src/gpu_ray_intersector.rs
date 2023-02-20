use std::{collections::HashMap, mem::size_of, rc::Rc};
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, BufferUsageFlags, DescriptorSetLayoutBinding,
    DescriptorType, MemoryPropertyFlags, ShaderStageFlags,
};

use crate::gpu_acceleration_structure::GpuTlas;

pub struct GpuIntersector {
    device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl GpuIntersector {
    pub fn new(device: Rc<DeviceContext>, _max_frames_in_flight: usize) -> Self {
        let shader_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_intersector.comp");
        let mut explicit_bindings = HashMap::new();
        let tlas_buffer_layout_binding = DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .build();
        let instance_buffer_layout_binding = DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .build();
        let ray_buffer_layout_binding = DescriptorSetLayoutBinding::builder()
            .binding(2)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .build();
        let intersection_buffer_layout_binding = DescriptorSetLayoutBinding::builder()
            .binding(3)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_BUFFER)
            .stage_flags(ShaderStageFlags::COMPUTE)
            .build();
        explicit_bindings.insert(
            0,
            vec![
                tlas_buffer_layout_binding,
                instance_buffer_layout_binding,
                ray_buffer_layout_binding,
                intersection_buffer_layout_binding,
            ],
        );
        let pipeline = ComputePipeline::new_from_source_file(
            shader_path.as_path(),
            device.clone(),
            1,
            "main",
            Some(explicit_bindings),
        )
        .unwrap();

        Self { pipeline, device }
    }

    pub fn intersect(&mut self, command_buffer: &mut CommandBuffer, width: usize, height: usize) {
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width as u32 / 16, height as u32 / 16, 1);
    }

    pub fn set(
        &mut self,
        ray_buffer: &BufferResource,
        intersection_result_buffer: &BufferResource,
        acceleration_structure: &GpuTlas,
    ) {
        self.pipeline
            .set_storage_buffer(0, 0, &acceleration_structure.tlas_buffer);
        self.pipeline
            .set_storage_buffer(0, 1, &acceleration_structure.instance_buffer);
        self.pipeline.set_storage_buffer(0, 2, ray_buffer);
        self.pipeline
            .set_storage_buffer(0, 3, intersection_result_buffer);
    }

    pub fn allocate_intersection_buffer(
        &self,
        width: usize,
        height: usize,
        host_visible: bool,
    ) -> BufferResource {
        let memory_flags = if host_visible {
            MemoryPropertyFlags::HOST_VISIBLE
        } else {
            MemoryPropertyFlags::DEVICE_LOCAL
        };
        BufferResource::new(
            self.device.clone(),
            self.intersection_buffer_size(width, height),
            memory_flags,
            BufferUsageFlags::STORAGE_BUFFER,
        )
    }

    pub fn intersection_buffer_size(&self, width: usize, height: usize) -> usize {
        width * height * size_of::<IntersectionResult>()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IntersectionResult {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    instance: u32,
    primitive: u32,
}
