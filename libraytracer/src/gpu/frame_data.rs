use vk_utils::buffer_resource::BufferResource;

use crate::types::UVec2;

pub struct FrameData {
    pub resolution: UVec2,
    pub uniform_buffer: BufferResource,
    pub ray_buffer: BufferResource,
    pub intersection_buffer: BufferResource,
}

impl FrameData {
    pub fn new(
        resolution: UVec2,
        uniform_buffer: BufferResource,
        ray_buffer: BufferResource,
        intersection_buffer: BufferResource,
    ) -> Self {
        Self {
            resolution,
            uniform_buffer,
            ray_buffer,
            intersection_buffer,
        }
    }
}
