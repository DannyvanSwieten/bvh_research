use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    image2d_resource::Image2DResource, pipeline_descriptor::ComputePipeline,
};

use super::frame_data::FrameData;

pub struct GpuRayAccumulator {
    pipeline: ComputePipeline,
}

impl GpuRayAccumulator {
    pub fn new(device: Rc<DeviceContext>, max_frames_in_flight: u32) -> Self {
        let shader_path = std::env::current_dir()
            .unwrap()
            .join("./intersect/assets/ray_accumulator.comp");

        let pipeline = ComputePipeline::new_from_source_file(
            &shader_path,
            device.clone(),
            max_frames_in_flight,
            "main",
            None,
        )
        .unwrap();

        Self { pipeline }
    }

    pub fn accumulate(&mut self, frame_data: &FrameData, command_buffer: &mut CommandBuffer) {
        let (x, y, z) = self.pipeline.workgroup_size();
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(
            frame_data.width as u32 / x,
            frame_data.height as u32 / y,
            1,
        );
    }

    pub fn set(&mut self, ray_buffer: &BufferResource, image: &Image2DResource) {
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
        self.pipeline.set_storage_image(0, 1, image);
    }
}
