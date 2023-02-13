use std::{collections::HashMap, mem::size_of, path::Path, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer,
    pipeline_descriptor::ComputePipeline, queue::CommandQueue, wait_handle::WaitHandle,
    BufferUsageFlags, DescriptorSetLayoutBinding, MemoryPropertyFlags,
};

use crate::types::Ray;

pub struct GpuRayGenerator {
    queue: Rc<CommandQueue>,
    pipeline: ComputePipeline,
}

impl GpuRayGenerator {
    pub fn new(
        queue: Rc<CommandQueue>,
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
            queue.device().clone(),
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
            .join("./assets/ray_gen.comp");

        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");

        let src = template_src + &src;

        let pipeline = ComputePipeline::new_from_source_string(
            queue.device().clone(),
            max_frames_in_flight,
            &src,
            "main",
            descriptors,
        )
        .unwrap();

        Self { queue, pipeline }
    }

    pub fn ray_buffer_size(&self, width: usize, height: usize) -> usize {
        width * height * size_of::<Ray>()
    }

    pub fn allocate_ray_buffer(
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
            self.queue.device(),
            self.ray_buffer_size(width, height),
            memory_flags,
            BufferUsageFlags::STORAGE_BUFFER,
        )
    }

    pub fn generate_rays(
        &mut self,
        width: usize,
        height: usize,
        ray_buffer: &BufferResource,
    ) -> WaitHandle {
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
        let mut command_buffer = CommandBuffer::new(self.queue.clone());
        command_buffer.begin();
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width as _, height as _, 1);
        command_buffer.submit()
    }
}