use std::{collections::HashMap, mem::size_of, path::Path, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, BufferUsageFlags, DescriptorSetLayoutBinding,
    MemoryPropertyFlags,
};

use crate::types::Ray;

use super::frame_data::FrameData;

pub struct GpuRayGenerator {
    device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl GpuRayGenerator {
    pub fn new(
        device: Rc<DeviceContext>,
        path: &Path,
        max_frames_in_flight: u32,
        descriptors: Option<HashMap<u32, Vec<DescriptorSetLayoutBinding>>>,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_gen.comp");

        let ray_gen_src = std::fs::read_to_string(path).expect("Couldn't load Ray generator file");
        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");
        let src = template_src + &ray_gen_src;

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

    pub fn new_from_string(
        device: Rc<DeviceContext>,
        src: &str,
        max_frames_in_flight: u32,
        descriptors: Option<HashMap<u32, Vec<DescriptorSetLayoutBinding>>>,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./intersect/assets/ray_gen.comp");

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

    pub fn ray_buffer_size(&self, frame_data: &FrameData) -> usize {
        frame_data.width * frame_data.height * size_of::<Ray>()
    }

    pub fn allocate_ray_buffer(
        &self,
        frame_data: &FrameData,
        host_visible: bool,
    ) -> BufferResource {
        let memory_flags = if host_visible {
            MemoryPropertyFlags::HOST_VISIBLE
        } else {
            MemoryPropertyFlags::DEVICE_LOCAL
        };
        BufferResource::new(
            self.device.clone(),
            self.ray_buffer_size(frame_data),
            memory_flags,
            BufferUsageFlags::STORAGE_BUFFER,
        )
    }

    pub fn generate_rays<T: Copy>(
        &mut self,
        command_buffer: &mut CommandBuffer,
        frame_data: &FrameData,
        constants: Option<&T>,
    ) {
        let (x, y, _z) = self.pipeline.workgroup_size();
        self.pipeline
            .set_uniform_buffer(0, 1, &frame_data.uniform_buffer);
        command_buffer.bind_compute_pipeline(&self.pipeline);

        if let Some(constants) = constants {
            command_buffer.push_compute_constants(&self.pipeline, 0, constants);
        }
        command_buffer.dispatch_compute(
            frame_data.width as u32 / x,
            frame_data.height as u32 / y,
            1,
        );
    }

    pub fn set_ray_buffer(&mut self, ray_buffer: &BufferResource) {
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
    }
}
