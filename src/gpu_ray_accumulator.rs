use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    image2d_resource::Image2DResource, image_resource::ImageResource,
    pipeline_descriptor::ComputePipeline, queue::CommandQueue, wait_handle::WaitHandle,
    AccessFlags, ImageLayout, PipelineStageFlags,
};

pub struct GpuRayAccumulator {
    device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl GpuRayAccumulator {
    pub fn new(device: Rc<DeviceContext>, max_frames_in_flight: u32) -> Self {
        let shader_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_accumulator.comp");

        let pipeline = ComputePipeline::new_from_source_file(
            &shader_path,
            device.clone(),
            max_frames_in_flight,
            "main",
            None,
        )
        .unwrap();

        Self { device, pipeline }
    }

    pub fn accumulate(
        &mut self,
        command_buffer: &mut CommandBuffer,
        ray_buffer: &BufferResource,
        image: &mut Image2DResource,
    ) {
        command_buffer.image_resource_transition(image, ImageLayout::GENERAL);
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
        self.pipeline.set_storage_image(0, 1, image);
        command_buffer.buffer_resource_barrier(
            &ray_buffer,
            PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::COMPUTE_SHADER,
            AccessFlags::MEMORY_WRITE,
            AccessFlags::MEMORY_READ,
        );
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(image.width(), image.height(), 1);
    }
}
