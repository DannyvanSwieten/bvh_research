use std::rc::Rc;

use vk_utils::{
    command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline,
};

use super::{
    gpu_acceleration_structure::GpuTlas,
    ray_tracing_pipeline_descriptor::{RayTracingPipelineDescriptor, ShaderSource},
};

pub struct RayTracingPipeline {
    _device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl RayTracingPipeline {
    pub fn new(device: Rc<DeviceContext>, descriptor: &RayTracingPipelineDescriptor) -> Self {
        let template_path = std::env::current_dir()
            .unwrap()
            .join("./intersect/assets/ray_uber_shader.comp");

        let template_src = std::fs::read_to_string(template_path)
            .expect("Couldn't load Ray generator template file");

        let ray_generation_shader = match &descriptor.ray_generation_source {
            ShaderSource::File(path) => {
                std::fs::read_to_string(path).expect("Couldn't load Ray generator file")
            }
            ShaderSource::String(src) => src.clone(),
        };

        let template_src =
            template_src.replace("___RAY_GENERATION_SHADER___", &ray_generation_shader);

        let closest_hit_shader = match &descriptor.closest_hit_shader_source {
            ShaderSource::File(path) => {
                std::fs::read_to_string(path).expect("Couldn't load Ray shader file")
            }
            ShaderSource::String(src) => src.clone(),
        };

        let template_src = template_src.replace("___CLOSEST_HIT_SHADER___", &closest_hit_shader);

        let any_hit_shader = match &descriptor.any_hit_shader_source {
            Some(ShaderSource::File(path)) => {
                std::fs::read_to_string(path).expect("Couldn't load Ray shader file")
            }
            Some(ShaderSource::String(src)) => src.clone(),
            None => "".to_string(),
        };

        let template_src = if any_hit_shader.is_empty() {
            template_src.replace("___ANY_HIT_SHADER_DEFINE___", "")
        } else {
            template_src.replace("___ANY_HIT_SHADER_DEFINE___", "#define ANY_HIT_SHADER\n")
        };

        let template_src = template_src.replace("___ANY_HIT_SHADER___", &any_hit_shader);

        let payload_content = descriptor
            .ray_payload_descriptor
            .attributes()
            .iter()
            .fold(String::new(), |acc, (name, data_type)| {
                acc + &format!("{} {};\n", data_type, name)
            });

        let template_src = template_src.replace("___RAY_PAYLOAD___", &payload_content);

        #[cfg(debug_assertions)]
        println!("{}", template_src);

        let pipeline = ComputePipeline::new_from_source_string(
            device.clone(),
            descriptor.max_frames_in_flight,
            &template_src,
            "main",
            None,
        )
        .expect("Couldn't create RayTracingPipeline");

        Self {
            _device: device.clone(),
            pipeline,
        }
    }

    // pub fn prepare_to_render(&self, width: u32, height: u32) -> FrameData {
    //     let mut uniform_buffer = BufferResource::new(
    //         self.device.clone(),
    //         size_of::<Vec2>(),
    //         MemoryPropertyFlags::HOST_VISIBLE,
    //         BufferUsageFlags::UNIFORM_BUFFER,
    //     );

    //     uniform_buffer.upload(&[width, height]);

    //     let ray_buffer = BufferResource::new(
    //         self.device.clone(),
    //         self.ray_payload_size * width as usize * height as usize,
    //         MemoryPropertyFlags::HOST_VISIBLE,
    //         BufferUsageFlags::STORAGE_BUFFER,
    //     );

    //     let intersection_buffer = BufferResource::new(
    //         self.device.clone(),
    //         self.intersection_payload_size * width as usize * height as usize,
    //         MemoryPropertyFlags::DEVICE_LOCAL,
    //         BufferUsageFlags::STORAGE_BUFFER,
    //     );

    //     FrameData::new(
    //         width as _,
    //         height as _,
    //         uniform_buffer,
    //         ray_buffer,
    //         intersection_buffer,
    //     )
    // }

    // pub fn set_shader_buffer(&mut self, set: usize, binding: usize, buffer: &BufferResource) {
    //     self.ray_shader.set_user_buffer(set, binding, buffer);
    // }

    pub fn trace<T: Copy>(
        &mut self,
        width: u32,
        height: u32,
        acceleration_structure: &GpuTlas,
        constants: Option<&T>,
        command_buffer: &mut CommandBuffer,
    ) {
        self.pipeline
            .set_storage_buffer(0, 0, acceleration_structure.buffer());
        self.pipeline
            .set_storage_buffer(0, 1, acceleration_structure.instance_buffer());

        command_buffer.bind_compute_pipeline(&self.pipeline);
        command_buffer.dispatch_compute(width, height, 1);
        // self.ray_generator.set_ray_buffer(&frame_data.ray_buffer);
        // self.ray_intersector.set(
        //     &frame_data.ray_buffer,
        //     &frame_data.intersection_buffer,
        //     acceleration_structure,
        // );
        // self.ray_shader.set(
        //     &frame_data.ray_buffer,
        //     &frame_data.intersection_buffer,
        //     acceleration_structure,
        // );

        // self.ray_generator
        //     .generate_rays(command_buffer, frame_data, constants);
        // command_buffer.buffer_resource_barrier(
        //     &frame_data.ray_buffer,
        //     PipelineStageFlags::COMPUTE_SHADER,
        //     PipelineStageFlags::COMPUTE_SHADER,
        //     AccessFlags::MEMORY_WRITE,
        //     AccessFlags::MEMORY_READ,
        // );
        // self.ray_intersector.intersect(command_buffer, frame_data);
        // command_buffer.buffer_resource_barrier(
        //     &frame_data.intersection_buffer,
        //     PipelineStageFlags::COMPUTE_SHADER,
        //     PipelineStageFlags::COMPUTE_SHADER,
        //     AccessFlags::MEMORY_WRITE,
        //     AccessFlags::MEMORY_READ,
        // );
        // self.ray_shader.shade_rays(command_buffer, frame_data);
        // command_buffer.buffer_resource_barrier(
        //     &frame_data.ray_buffer,
        //     PipelineStageFlags::COMPUTE_SHADER,
        //     PipelineStageFlags::COMPUTE_SHADER,
        //     AccessFlags::MEMORY_READ,
        //     AccessFlags::MEMORY_WRITE,
        // );
    }
}
