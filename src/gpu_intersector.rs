use std::{collections::HashMap, rc::Rc};
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, queue::CommandQueue, wait_handle::WaitHandle,
    DescriptorSetLayoutBinding, DescriptorType, QueueFlags, ShaderStageFlags,
};

use crate::gpu_acceleration_structure::GpuTlas;

pub struct GpuIntersector {
    device: Rc<DeviceContext>,
    queue: Rc<CommandQueue>,
    pipeline: ComputePipeline,
}

#[repr(C)]
struct IntersectionResult {
    t: f32,
    instance: u32,
    primitive: u32,
}

impl GpuIntersector {
    pub fn new(device: Rc<DeviceContext>, max_frames_in_flight: usize) -> Self {
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
        explicit_bindings.insert(
            0,
            vec![tlas_buffer_layout_binding, instance_buffer_layout_binding],
        );
        let pipeline = ComputePipeline::new_from_source_file(
            shader_path.as_path(),
            device.clone(),
            1,
            "main",
            Some(explicit_bindings),
        )
        .unwrap();

        let queue = Rc::new(CommandQueue::new(device.clone(), QueueFlags::COMPUTE));
        Self {
            device,
            pipeline,
            queue,
        }
    }

    pub fn intersect(
        &mut self,
        width: usize,
        height: usize,
        ray_buffer: &BufferResource,
        intersection_result_buffer: &BufferResource,
        acceleration_structure: &GpuTlas,
    ) -> WaitHandle {
        let mut command_buffer = CommandBuffer::new(self.queue.clone());
        self.pipeline
            .set_storage_buffer(0, 0, &acceleration_structure.tlas_buffer);
        self.pipeline
            .set_storage_buffer(0, 1, &acceleration_structure.instance_buffer);
        self.pipeline.set_storage_buffer(0, 2, &ray_buffer);
        self.pipeline
            .set_storage_buffer(0, 3, &intersection_result_buffer);

        command_buffer.begin();
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width as u32, height as u32, 1);
        command_buffer.submit()
    }
}
