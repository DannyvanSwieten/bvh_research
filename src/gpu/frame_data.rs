use vk_utils::buffer_resource::BufferResource;

pub struct FrameData {
    pub width: usize,
    pub height: usize,
    pub uniform_buffer: BufferResource,
}

impl FrameData {
    pub fn new(width: usize, height: usize, uniform_buffer: BufferResource) -> Self {
        Self {
            width,
            height,
            uniform_buffer,
        }
    }
}
