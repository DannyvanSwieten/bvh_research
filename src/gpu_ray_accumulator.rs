use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    image2d_resource::Image2DResource, pipeline_descriptor::ComputePipeline,
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

    pub fn accumulate(&mut self, width: usize, height: usize, command_buffer: &mut CommandBuffer) {
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width as u32, height as u32, 1);
    }

    pub fn set(&mut self, ray_buffer: &BufferResource, image: &Image2DResource) {
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
        self.pipeline.set_storage_image(0, 1, image);
    }
}
