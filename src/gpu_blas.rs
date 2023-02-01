use std::rc::Rc;

use cgmath::SquareMatrix;
use vk_utils::{
    buffer_resource::BufferResource, device_context::DeviceContext, BufferUsageFlags,
    MemoryPropertyFlags,
};

use crate::{bvh::Bvh, types::Mat4};

pub struct GpuBlas {
    buffer: BufferResource,
    pub bvh: Rc<Bvh>,
}

impl GpuBlas {
    pub fn new(device: Rc<DeviceContext>, bvh: Rc<Bvh>) -> Self {
        let mut buffer = BufferResource::new(
            device,
            bvh.size(),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        );
        buffer.upload(bvh.nodes());
        Self { buffer, bvh }
    }

    pub fn address(&self) -> u64 {
        self.buffer.device_address()
    }
}

pub struct GpuInstanceProxy {
    pub blas: Rc<GpuBlas>,
    pub instance_id: u32,
    pub transform: Mat4,
}

impl GpuInstanceProxy {
    pub fn new(blas: Rc<GpuBlas>, instance_id: u32) -> Self {
        Self {
            blas,
            instance_id,
            transform: Mat4::identity(),
        }
    }

    pub fn with_transform(mut self, t: Mat4) -> Self {
        self.transform = t;
        self
    }

    pub fn transformed(&mut self, t: Mat4) {
        self.transform = self.transform * t
    }
}
