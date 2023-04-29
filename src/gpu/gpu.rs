use std::{mem::size_of, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, device_context::DeviceContext, vulkan::Vulkan,
    BufferUsageFlags, DebugUtils, MemoryPropertyFlags, PhysicalDeviceFeatures2KHR,
    PhysicalDeviceVulkan12Features,
};

use crate::types::{Ray, UVec2, Vec2};

use super::{frame_data::FrameData, gpu_ray_intersector::IntersectionResult};

pub struct Gpu {
    _vulkan: Rc<Vulkan>,
    physical_devices: Vec<vk_utils::gpu::Gpu>,
}

impl Gpu {
    pub fn new(application_name: &str) -> Self {
        let vulkan = Vulkan::new(
            application_name,
            &[],
            &[DebugUtils::name().to_str().unwrap()],
        );

        let physical_devices = vulkan
            .physical_devices()
            .into_iter()
            .flat_map(|device| {
                if device.supports_compute() {
                    Some(device)
                } else {
                    None
                }
            })
            .collect();

        Self {
            _vulkan: Rc::new(vulkan),
            physical_devices,
        }
    }

    pub fn physical_devices(&self) -> &[vk_utils::gpu::Gpu] {
        &self.physical_devices
    }

    pub fn create_device(&self, index: usize) -> DeviceContext {
        let gpu = &self.physical_devices[index];
        let mut address_features = PhysicalDeviceVulkan12Features::builder()
            .buffer_device_address(true)
            .shader_input_attachment_array_dynamic_indexing(true)
            .descriptor_indexing(true)
            .runtime_descriptor_array(true)
            .build();
        let mut features2 = PhysicalDeviceFeatures2KHR::default();
        unsafe {
            gpu.vulkan()
                .vk_instance()
                .get_physical_device_features2(*gpu.vk_physical_device(), &mut features2);
        }
        gpu.device_context_builder(&["VK_KHR_buffer_device_address"], |builder| {
            builder
                .push_next(&mut address_features)
                .enabled_features(&features2.features)
        })
    }

    pub fn create_frame_data(
        &self,
        device_context: Rc<DeviceContext>,
        resolution: UVec2,
    ) -> FrameData {
        let mut uniform_buffer = BufferResource::new(
            device_context.clone(),
            size_of::<Vec2>(),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::UNIFORM_BUFFER,
        );

        uniform_buffer.upload(&[resolution]);

        let ray_buffer = BufferResource::new(
            device_context.clone(),
            size_of::<Ray>() * resolution.x as usize * resolution.y as usize,
            MemoryPropertyFlags::DEVICE_LOCAL,
            BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        );

        let intersection_buffer = BufferResource::new(
            device_context.clone(),
            size_of::<IntersectionResult>() * resolution.x as usize * resolution.y as usize,
            MemoryPropertyFlags::DEVICE_LOCAL,
            BufferUsageFlags::STORAGE_BUFFER | BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        );

        FrameData::new(resolution, uniform_buffer, ray_buffer, intersection_buffer)
    }
}
