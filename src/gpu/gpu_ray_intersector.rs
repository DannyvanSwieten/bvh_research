use std::{collections::HashMap, rc::Rc};
use vk_utils::{
    command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, AccessFlags, DescriptorSetLayoutBinding, DescriptorType,
    PipelineStageFlags, ShaderStageFlags,
};

use super::{frame_data::FrameData, gpu_acceleration_structure::GpuTlas};

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

    pub fn intersect(
        &mut self,
        command_buffer: &mut CommandBuffer,
        frame_data: &FrameData,
        acceleration_structure: &GpuTlas,
    ) {
        self.set(frame_data, acceleration_structure);
        self.pipeline
            .set_uniform_buffer(0, 4, &frame_data.uniform_buffer);
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(
            frame_data.resolution.x / 16,
            frame_data.resolution.y / 16,
            1,
        );
        command_buffer.buffer_resource_barrier(
            &frame_data.ray_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );
    }

    fn set(&mut self, frame_data: &FrameData, acceleration_structure: &GpuTlas) {
        self.pipeline
            .set_storage_buffer(0, 0, &acceleration_structure.tlas_buffer);
        self.pipeline
            .set_storage_buffer(0, 1, &acceleration_structure.instance_buffer);
        self.pipeline
            .set_storage_buffer(0, 2, &frame_data.ray_buffer);
        self.pipeline
            .set_storage_buffer(0, 3, &frame_data.intersection_buffer);
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
