use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, device_context::DeviceContext, BufferUsageFlags,
    MemoryPropertyFlags,
};

use crate::{bvh::Bvh, types::AABB};

pub struct TriangleGeometry {
    aabb: AABB,
    _vertex_buffer: u64,
    _index_buffer: u64,
    _triangle_buffer: BufferResource,
    blas_buffer: BufferResource,
}

impl TriangleGeometry {
    pub fn new(
        device: Rc<DeviceContext>,
        vertex_buffer: &BufferResource,
        index_buffer: &BufferResource,
    ) -> Self {
        let bvh = Bvh::new(&vertex_buffer.copy_data(), &index_buffer.copy_data());
        let mut triangle_buffer = BufferResource::new(
            device.clone(),
            std::mem::size_of_val(bvh.triangles()),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        );
        triangle_buffer.upload(bvh.triangles());

        let mut blas_buffer = BufferResource::new(
            device,
            bvh.size() + 24,
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        );
        blas_buffer.upload(&[vertex_buffer.device_address()]);
        blas_buffer.upload_at(8, &[index_buffer.device_address()]);
        blas_buffer.upload_at(16, &[triangle_buffer.device_address()]);
        blas_buffer.upload_at(24, bvh.nodes());

        Self {
            aabb: *bvh.aabb(),
            _vertex_buffer: vertex_buffer.device_address(),
            _index_buffer: index_buffer.device_address(),
            _triangle_buffer: triangle_buffer,
            blas_buffer,
        }
    }

    pub fn aabb(&self) -> &AABB {
        &self.aabb
    }

    pub fn address(&self) -> u64 {
        self.blas_buffer.device_address()
    }
}
