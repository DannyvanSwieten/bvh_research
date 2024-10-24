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

pub struct BufferDescriptor {
    name: String,
    attributes: HashMap<String, DataType>,
    read_only: bool,
}

impl BufferDescriptor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attributes: HashMap::new(),
            read_only: false,
        }
    }

    pub fn with_attribute(mut self, name: &str, data_type: DataType) -> Self {
        self.attributes.insert(name.to_string(), data_type);
        self
    }

    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
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

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct RayTracingPipelineDescriptor {
    pub ray_payload_descriptor: PayloadDescriptor,
    pub ray_generation_source: ShaderSource,
    pub closest_hit_shader_source: ShaderSource,
    pub miss_shader_sources: Vec<ShaderSource>,
    pub any_hit_shader_source: Option<ShaderSource>,
    pub max_frames_in_flight: u32,
    pub intersection_functions: Vec<ShaderSource>,
    pub buffers: Vec<BufferDescriptor>,
}

impl RayTracingPipelineDescriptor {
    pub fn new(
        ray_generation_source: ShaderSource,
        closest_hit_shader_source: ShaderSource,
    ) -> Self {
        Self {
            ray_payload_descriptor: PayloadDescriptor::new(),
            ray_generation_source,
            closest_hit_shader_source,
            miss_shader_sources: Vec::new(),
            any_hit_shader_source: None,
            max_frames_in_flight: 1,
            intersection_functions: Vec::new(),
            buffers: Vec::new(),
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

    pub fn with_intersection_function(mut self, source: ShaderSource) -> Self {
        self.intersection_functions.push(source);
        self
    }

    pub fn with_intersection_functions(mut self, sources: Vec<ShaderSource>) -> Self {
        self.intersection_functions.extend(sources);
        self
    }

    pub fn ray_payload_descriptor(&self) -> &PayloadDescriptor {
        &self.ray_payload_descriptor
    }

    pub fn intersection_functions(&self) -> &[ShaderSource] {
        &self.intersection_functions
    }

    pub fn with_buffer_descriptor(mut self, descriptor: BufferDescriptor) -> Self {
        self.buffers.push(descriptor);
        self
    }
}
