use std::{path::Path, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline,
};

use super::{frame_data::FrameData, gpu_acceleration_structure::GpuTlas};

pub struct GpuRayShader {
    pipeline: ComputePipeline,
}

impl GpuRayShader {
    pub fn new(
        device: Rc<DeviceContext>,
        path: &Path,
        ray_struct: &str,
        intersection_struct: &str,
        max_frames_in_flight: u32,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./assets/ray_shader.comp");

        let ray_gen_src = std::fs::read_to_string(path).expect("Couldn't load Ray generator file");
        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");
        let template_src = template_src.replace("___INTERSECTION_STRUCT___", intersection_struct);
        let template_src = template_src.replace("___RAY_STRUCT___", ray_struct);
        let src = template_src + &ray_gen_src;

        #[cfg(debug_assertions)]
        println!("Ray Shader: {}", src);

        let pipeline = ComputePipeline::new_from_source_string(
            device.clone(),
            max_frames_in_flight,
            &src,
            "main",
            None,
        )
        .unwrap();

        Self { pipeline }
    }

    pub fn new_from_string(
        device: Rc<DeviceContext>,
        src: &str,
        ray_struct: &str,
        intersection_struct: &str,
        max_frames_in_flight: u32,
    ) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./intersect/assets/ray_shader.comp");

        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");

        let template_src = template_src.replace("___INTERSECTION_STRUCT___", intersection_struct);
        let template_src = template_src.replace("___RAY_STRUCT___", ray_struct);

        let src = template_src + src;

        let pipeline = ComputePipeline::new_from_source_string(
            device.clone(),
            max_frames_in_flight,
            &src,
            "main",
            None,
        )
        .unwrap();

        Self { pipeline }
    }

    pub fn shade_rays(&mut self, command_buffer: &mut CommandBuffer, frame_data: &FrameData) {
        let (x, y, _z) = self.pipeline.workgroup_size();
        self.pipeline
            .set_uniform_buffer(0, 3, &frame_data.uniform_buffer);
        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(
            frame_data.width as u32 / x,
            frame_data.height as u32 / y,
            1,
        );
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
