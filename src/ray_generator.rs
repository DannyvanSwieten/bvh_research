use std::rc::Rc;

use vk_utils::{buffer_resource::BufferResource, device_context::DeviceContext};

use crate::types::Ray;

pub trait RayGenerator {
    type RayBuffer;
    fn allocate_rays(&self, width: usize, height: usize);
    fn generate_rays(&self, width: usize, height: usize) -> &Self::RayBuffer;
}

pub struct GpuRayGenerator {
    device: Rc<DeviceContext>,
    ray_buffer: BufferResource,
}

impl GpuRayGenerator {
    pub fn new(width: usize, height: usize) {}
}

impl RayGenerator for GpuRayGenerator {
    type RayBuffer = BufferResource;

    fn allocate_rays(&self, width: usize, height: usize) {
        todo!()
    }

    fn generate_rays(&self, width: usize, height: usize) -> &Self::RayBuffer {
        &self.ray_buffer
    }
}
