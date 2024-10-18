use std::{collections::HashMap, path::PathBuf};

use crate::types::DataType;

pub enum ShaderSource {
    File(PathBuf),
    String(String),
}

pub struct PayloadDescriptor {
    attributes: HashMap<String, DataType>,
}

impl PayloadDescriptor {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn with_attribute(mut self, name: &str, data_type: DataType) -> Self {
        self.attributes.insert(name.to_string(), data_type);
        self
    }

    pub fn attributes(&self) -> &HashMap<String, DataType> {
        &self.attributes
    }

    pub fn byte_size(&self) -> usize {
        self.attributes
            .values()
            .fold(0, |acc, data_type| acc + data_type.byte_size())
    }
}

impl Default for PayloadDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RayTracingPipelineDescriptor {
    pub ray_payload_descriptor: PayloadDescriptor,
    pub intersection_payload_descriptor: PayloadDescriptor,
    pub ray_generation_source: ShaderSource,
    pub ray_shader_source: ShaderSource,
    pub max_frames_in_flight: u32,
}

impl RayTracingPipelineDescriptor {
    pub fn new(ray_generation_source: ShaderSource, ray_shader_source: ShaderSource) -> Self {
        Self {
            ray_payload_descriptor: PayloadDescriptor::new(),
            intersection_payload_descriptor: PayloadDescriptor::new(),
            ray_generation_source,
            ray_shader_source,
            max_frames_in_flight: 1,
        }
    }

    pub fn with_max_frames_in_flight(mut self, max_frames_in_flight: u32) -> Self {
        self.max_frames_in_flight = max_frames_in_flight;
        self
    }

    pub fn with_ray_payload_descriptor(mut self, descriptor: PayloadDescriptor) -> Self {
        self.ray_payload_descriptor = descriptor;
        self
    }

    pub fn with_intersection_payload_descriptor(mut self, descriptor: PayloadDescriptor) -> Self {
        self.intersection_payload_descriptor = descriptor;
        self
    }

    pub fn ray_payload_descriptor(&self) -> &PayloadDescriptor {
        &self.ray_payload_descriptor
    }

    pub fn intersection_payload_descriptor(&self) -> &PayloadDescriptor {
        &self.intersection_payload_descriptor
    }
}
