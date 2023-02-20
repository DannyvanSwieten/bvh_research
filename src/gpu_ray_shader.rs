use std::{collections::HashMap, path::Path, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, DescriptorSetLayoutBinding,
};

use crate::gpu_acceleration_structure::GpuTlas;

pub struct GpuRayShader {
    device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl GpuRayShader {
    pub fn new(
        device: Rc<DeviceContext>,
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
            .join("./assets/ray_shader.comp");

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

    pub fn shade_rays(&mut self, command_buffer: &mut CommandBuffer, width: usize, height: usize) {
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width as u32 / 16, height as u32 / 16, 1);
    }

    pub fn set(
        &mut self,
        ray_buffer: &BufferResource,
        intersection_buffer: &BufferResource,
        acceleration_structure: &GpuTlas,
    ) {
        self.pipeline.set_storage_buffer(0, 0, ray_buffer);
        self.pipeline.set_storage_buffer(0, 1, intersection_buffer);
        self.pipeline
            .set_storage_buffer(0, 2, &acceleration_structure.instance_buffer);
    }

    pub fn set_user_buffer(&mut self, set: usize, binding: usize, buffer: &BufferResource) {
        // set 0 is not for the user
        assert_ne!(set, 0);
        self.pipeline.set_storage_buffer(set, binding, buffer)
    }
}
