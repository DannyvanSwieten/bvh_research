use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    image2d_resource::Image2DResource, pipeline_descriptor::ComputePipeline,
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

        let payload_content = descriptor.ray_payload_descriptor.attributes().iter().fold(
            String::new(),
            |acc, (name, att)| {
                if att.is_array && att.array_size == 0 {
                    acc + &format!("{} {}[];\n", att.data_type, name)
                } else if att.is_array {
                    acc + &format!("{} {}[{}];\n", att.data_type, name, att.array_size)
                } else {
                    acc + &format!("{} {};\n", att.data_type, name)
                }
            },
        );

        let template_src = template_src.replace("___RAY_PAYLOAD___", &payload_content);

        let buffer_bindings =
            descriptor
                .buffers
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (index, binding)| {
                    let read_only = if binding.read_only() { "readonly" } else { "" };
                    let buffer_string = acc
                        + &format!(
                            "layout(scalar, set = 1, binding = {}) {} buffer {} {{\n",
                            index,
                            read_only,
                            binding.name(),
                        );

                    let buffer_string =
                        binding
                            .attributes()
                            .iter()
                            .fold(buffer_string, |acc, (name, att)| {
                                if att.is_array && att.array_size == 0 {
                                    acc + &format!("    {} {}[];\n", att.data_type, name)
                                } else if att.is_array {
                                    acc + &format!(
                                        "   {} {}[{}];\n",
                                        att.data_type, name, att.array_size
                                    )
                                } else {
                                    acc + &format!("    {} {};\n", att.data_type, name)
                                }
                            });

                    buffer_string + "\n};\n"
                });

        let image_bindings =
            descriptor
                .images
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (index, binding)| {
                    let data_type = if binding.is_float { "f" } else { "" };
                    let is_read_only = if binding.is_read_only { "readonly" } else { "" };
                    acc + &format!(
                        "layout(set = 2, binding = {}, rgba{}{}) uniform  {} image2D {};\n",
                        index, binding.bits_per_channel, data_type, is_read_only, binding.name
                    )
                });

        let template_src = template_src.replace("___IMAGE_BINDINGS___", &image_bindings);

        let template_src = template_src.replace("___BUFFER_BINDINGS___", &buffer_bindings);

        let miss_shader_sources: Vec<String> = descriptor
            .miss_shader_sources
            .iter()
            .map(|src| match &src.source {
                ShaderSource::File(path) => {
                    std::fs::read_to_string(path).expect("Couldn't load Ray shader file")
                }
                ShaderSource::String(src) => src.clone(),
            })
            .collect();

        let miss_shaders = miss_shader_sources
            .iter()
            .fold(String::new(), |acc, src| acc + src + "\n");

        let template_src = template_src.replace("___MISS_SHADERS___", &miss_shaders);

        let miss_shader_invocations =
            descriptor.miss_shader_sources.iter().enumerate().fold(
                String::new(),
                |acc, (index, src)| {
                    acc + &format!("case {}: {}(ray, payload); \n", index, &src.name)
                },
            ) + "default: break;";

        let template_src =
            template_src.replace("___MISS_SHADER_INVOCATIONS___", &miss_shader_invocations);

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

    pub fn set_storage_buffer(&mut self, location: usize, buffer: &BufferResource) {
        self.pipeline.set_storage_buffer(1, location, buffer);
    }

    pub fn set_storage_image(&mut self, location: usize, image: &Image2DResource) {
        self.pipeline.set_storage_image(2, location, image);
    }
}
