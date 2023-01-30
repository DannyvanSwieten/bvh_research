use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, device_context::DeviceContext, BufferUsageFlags,
    MemoryPropertyFlags,
};

use crate::top_level_acceleration_structure::TopLevelAccelerationStructure;

pub struct GpuAccelerationStructure {
    device: Rc<DeviceContext>,
    blas_buffers: Vec<BufferResource>,
    tlas_buffer: BufferResource,
    instance_buffer: BufferResource,
}

impl GpuAccelerationStructure {
    pub fn new(device: Rc<DeviceContext>, tlas: &TopLevelAccelerationStructure) {
        let blas_buffers = tlas
            .instances()
            .iter()
            .map(|instance| {
                let nodes = instance.blas.nodes();
                let mut buffer = BufferResource::new(
                    device.clone(),
                    instance.blas.size(),
                    MemoryPropertyFlags::HOST_VISIBLE,
                    BufferUsageFlags::STORAGE_BUFFER,
                );
                buffer.upload(nodes);
                buffer
            })
            .collect();
        let mut tlas_buffer = BufferResource::new(
            device.clone(),
            tlas.size(),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );

        tlas_buffer.upload(tlas.nodes());

        let instance_buffer = BufferResource::new(
            device.clone(),
            tlas.size(),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );

        Self {
            device: device.clone(),
            blas_buffers,
            tlas_buffer,
            instance_buffer,
        };
    }
}
