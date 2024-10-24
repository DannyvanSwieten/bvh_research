use std::{mem::size_of, rc::Rc};
use vk_utils::{
    buffer_resource::BufferResource, command_buffer::CommandBuffer, device_context::DeviceContext,
    pipeline_descriptor::ComputePipeline, BufferUsageFlags, MemoryPropertyFlags,
};

use super::{frame_data::FrameData, gpu_acceleration_structure::GpuTlas};

pub struct IntersectionFunction {
    pub name: String,
    pub code: String,
}

impl IntersectionFunction {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
        }
    }
}

pub struct GpuIntersector {
    device: Rc<DeviceContext>,
    pipeline: ComputePipeline,
}

impl GpuIntersector {
    pub fn new(
        device: Rc<DeviceContext>,
        ray_struct: &str,
        intersection_struct: &str,
        intersection_table: &[IntersectionFunction],
        _max_frames_in_flight: usize,
    ) -> Self {
        let shader_path = std::env::current_dir()
            .unwrap()
            .join("./intersect/assets/ray_intersector.comp");

        // load the file as string
        let shader = std::fs::read_to_string(shader_path.as_path()).unwrap();
        let intersection_code = intersection_table
            .iter()
            .fold(String::new(), |acc, x| acc + &x.name + "\n");

        let intersection_code = intersection_table
            .iter()
            .fold(intersection_code, |acc, x| acc + &x.code + "\n");

        let shader = shader.replace("___CUSTOM_INTERSECTION_FUNCTIONS___", &intersection_code);
        let shader = shader.replace("___RAY_STRUCT___", ray_struct);
        let shader = shader.replace("___INTERSECTION_STRUCT___", intersection_struct);

        let shader = if !intersection_table.is_empty() {
            let cases = "match(instance_flags) {\n".to_string()
                + &intersection_table
                    .iter()
                    .enumerate()
                    .fold(String::new(), |acc, (i, x)| {
                        acc + &format!("{} => return {}(ray);\n", i, x.name)
                    })
                + "_ => break,\n}";

            shader.replace("___INTERSECTION_CASES___", &cases)
        } else {
            shader.replace("___INTERSECTION_CASES___", "")
        };

        #[cfg(debug_assertions)]
        println!("Ray Intersector: {}", shader);

        let pipeline =
            ComputePipeline::new_from_source_string(device.clone(), 1, &shader, "main", None)
                .unwrap();

        Self { pipeline, device }
    }

    pub fn intersect(&mut self, command_buffer: &mut CommandBuffer, frame_data: &FrameData) {
        let (x, y, _z) = self.pipeline.workgroup_size();
        self.pipeline
            .set_uniform_buffer(0, 4, &frame_data.uniform_buffer);
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
        intersection_result_buffer: &BufferResource,
        acceleration_structure: &GpuTlas,
    ) {
        self.pipeline
            .set_storage_buffer(0, 0, &acceleration_structure.buffer());
        self.pipeline
            .set_storage_buffer(0, 1, &acceleration_structure.buffer());
        self.pipeline.set_storage_buffer(0, 2, ray_buffer);
        self.pipeline
            .set_storage_buffer(0, 3, intersection_result_buffer);
    }

    pub fn allocate_intersection_buffer(
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
            self.device.clone(),
            self.intersection_buffer_size(width, height),
            memory_flags,
            BufferUsageFlags::STORAGE_BUFFER,
        )
    }

    pub fn intersection_buffer_size(&self, width: usize, height: usize) -> usize {
        width * height * size_of::<IntersectionResult>()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IntersectionResult {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    instance: u32,
    primitive: u32,
}
