use std::rc::Rc;

use vk_utils::{
    command_buffer::CommandBuffer, device_context::DeviceContext,
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
            .join("./assets/ray_accumulator.comp");

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

    pub fn accumulate(
        &mut self,
        command_buffer: &mut CommandBuffer,
        frame_data: &FrameData,
        image: &mut Image2DResource,
    ) {
        self.pipeline
            .set_storage_buffer(0, 0, &frame_data.ray_buffer);
        self.pipeline.set_storage_image(0, 1, image);
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(
            frame_data.resolution.x as u32,
            frame_data.resolution.y as u32,
            1,
        );
    }
}
