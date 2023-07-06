use std::{collections::HashMap, rc::Rc};

use vk_utils::{
    command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, AccessFlags, DescriptorSetLayoutBinding,
    PipelineStageFlags,
};

use crate::ray_tracer::shader_module::ShaderModule;

use super::frame_data::FrameData;

pub struct GpuRayGenerator {
    device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl GpuRayGenerator {
    pub fn new(
        device: Rc<DeviceContext>,
        shader_module: &ShaderModule,
        max_frames_in_flight: u32,
        descriptors: Option<HashMap<u32, Vec<DescriptorSetLayoutBinding>>>,
    ) -> Self {
        Self::new_from_string(
            device,
            shader_module.source(),
            max_frames_in_flight,
            descriptors,
        )
    }

    fn new_from_string(
        device: Rc<DeviceContext>,
        src: &str,
        max_frames_in_flight: u32,
        descriptors: Option<HashMap<u32, Vec<DescriptorSetLayoutBinding>>>,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_gen.comp");

        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");

        let src = template_src + src;

        let pipeline = ComputePipeline::new_from_source_string(
            device.clone(),
            max_frames_in_flight,
            &src,
            "main",
            descriptors,
        )
        .unwrap();

        Self { device, pipeline }
    }

    pub fn generate_rays<T: Copy>(
        &mut self,
        command_buffer: &mut CommandBuffer,
        frame_data: &FrameData,
        constants: Option<&T>,
    ) {
        self.pipeline
            .set_storage_buffer(0, 0, &frame_data.ray_buffer);
        self.pipeline
            .set_uniform_buffer(0, 1, &frame_data.uniform_buffer);
        command_buffer.bind_compute_pipeline(&self.pipeline);

        if let Some(constants) = constants {
            command_buffer.push_compute_constants(&self.pipeline, 0, constants);
        }
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
}
