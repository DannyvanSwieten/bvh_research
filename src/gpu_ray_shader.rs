use std::{collections::HashMap, path::Path, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    pipeline_descriptor::ComputePipeline, queue::CommandQueue, wait_handle::WaitHandle,
    DescriptorSetLayoutBinding,
};

use crate::gpu_acceleration_structure::GpuTlas;

pub struct GpuRayShader {
    queue: Rc<CommandQueue>,
    pipeline: ComputePipeline,
}

impl GpuRayShader {
    pub fn new(
        queue: Rc<CommandQueue>,
        path: &Path,
        max_frames_in_flight: u32,
        descriptors: Option<HashMap<u32, Vec<DescriptorSetLayoutBinding>>>,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_shader.comp");

        let ray_gen_src = std::fs::read_to_string(path).expect("Couldn't load Ray generator file");
        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");
        let src = template_src + &ray_gen_src;

        let pipeline = ComputePipeline::new_from_source_string(
            queue.device(),
            max_frames_in_flight,
            &src,
            "main",
            descriptors,
        )
        .unwrap();

        Self { queue, pipeline }
    }

    pub fn new_from_string(
        queue: Rc<CommandQueue>,
        src: &str,
        max_frames_in_flight: u32,
        descriptors: Option<HashMap<u32, Vec<DescriptorSetLayoutBinding>>>,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_shader.comp");

        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");

        let src = template_src + src;

        let pipeline = ComputePipeline::new_from_source_string(
            queue.device(),
            max_frames_in_flight,
            &src,
            "main",
            descriptors,
        )
        .unwrap();

        Self { queue, pipeline }
    }

    pub fn shade_rays(
        &mut self,
        width: usize,
        height: usize,
        ray_buffer: &BufferResource,
        intersection_buffer: &BufferResource,
        acceleration_structure: &GpuTlas,
    ) -> WaitHandle {
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
        self.pipeline.set_storage_buffer(0, 1, intersection_buffer);
        self.pipeline
            .set_storage_buffer(0, 2, &acceleration_structure.instance_buffer);
        let mut command_buffer = CommandBuffer::new(self.queue.clone());
        command_buffer.begin();
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width as u32 / 8, height as u32 / 8, 1);
        command_buffer.submit()
    }
}
