use vk_utils::buffer_resource::BufferResource;

pub struct FrameData {
    pub width: usize,
    pub height: usize,
    pub uniform_buffer: BufferResource,
    pub ray_buffer: BufferResource,
    pub intersection_buffer: BufferResource,
}

impl FrameData {
    pub fn new(
        width: usize,
        height: usize,
        uniform_buffer: BufferResource,
        ray_buffer: BufferResource,
        intersection_buffer: BufferResource,
    ) -> Self {
        Self {
            width,
            height,
            uniform_buffer,
            ray_buffer,
            intersection_buffer,
        }
    }
}
