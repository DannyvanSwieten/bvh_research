use std::rc::Rc;

use vk_utils::{
    device_context::DeviceContext, vulkan::Vulkan, PhysicalDeviceFeatures2KHR,
    PhysicalDeviceVulkan12Features,
};

pub struct Gpu {
    _vulkan: Rc<Vulkan>,
    physical_devices: Vec<vk_utils::gpu::Gpu>,
}

impl Gpu {
    pub fn new(application_name: &str) -> Self {
        let vulkan = Vulkan::new(application_name, &[], &[]);

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
        let mut address_features = PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .shader_input_attachment_array_dynamic_indexing(true)
            .descriptor_indexing(true)
            .runtime_descriptor_array(true);
        let mut features2 = PhysicalDeviceFeatures2KHR::default();
        unsafe {
            gpu.vulkan()
                .vk_instance()
                .get_physical_device_features2(*gpu.vk_physical_device(), &mut features2);
        }
        // turn CStr into a string
        gpu.device_context_builder(
            &[vk_utils::buffer_device_address::NAME.to_str().unwrap()],
            |builder| {
                builder
                    .push_next(&mut address_features)
                    .enabled_features(&features2.features)
            },
        )
    }
}
