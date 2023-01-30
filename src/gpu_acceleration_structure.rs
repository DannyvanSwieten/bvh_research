use std::rc::Rc;

use vk_utils::{
    buffer_resource::BufferResource, device_context::DeviceContext, BufferUsageFlags,
    MemoryPropertyFlags,
};

use crate::{
    acc_bvh_midpoint_split::AccMidPointSplit,
    bottom_level_acceleration_structure::AccelerationStructure,
    top_level_acceleration_structure::TopLevelAccelerationStructure,
};

pub struct GpuAccelerationStructure {
    device: Rc<DeviceContext>,
    blas_buffers: Vec<BufferResource>,
    tlas_buffer: BufferResource,
}

impl GpuAccelerationStructure {
    pub fn new(device: Rc<DeviceContext>, tlas: &TopLevelAccelerationStructure) {
        Self {
            device: device.clone(),
            blas_buffers: tlas
                .instances()
                .iter()
                .map(|instance| {
                    let nodes = instance.blas.nodes();
                    BufferResource::new(
                        device.clone(),
                        64,
                        MemoryPropertyFlags::HOST_VISIBLE,
                        BufferUsageFlags::STORAGE_BUFFER,
                    )
                })
                .collect(),
            tlas_buffer: BufferResource::new(
                device.clone(),
                64,
                MemoryPropertyFlags::HOST_VISIBLE,
                BufferUsageFlags::STORAGE_BUFFER,
            ),
        };
    }
}
