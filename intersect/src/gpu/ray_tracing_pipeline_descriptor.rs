use std::{collections::HashMap, path::PathBuf};

use crate::types::DataType;

pub enum ShaderSource {
    File(PathBuf),
    String(String),
}

pub struct ShaderFunction {
    pub source: ShaderSource,
    pub name: String,
}

impl ShaderFunction {
    pub fn new(source: ShaderSource, name: &str) -> Self {
        Self {
            source,
            name: name.to_string(),
        }
    }
}

pub struct StructAttribute {
    pub data_type: DataType,
    pub is_array: bool,
    pub array_size: usize, // if 0 and is_array is true, then it's a dynamically array
}

impl Default for StructAttribute {
    fn default() -> Self {
        Self {
            data_type: DataType::Float,
            is_array: false,
            array_size: 0,
        }
    }
}

pub struct PayloadDescriptor {
    attributes: HashMap<String, StructAttribute>,
}

impl PayloadDescriptor {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn with_attribute(mut self, name: &str, attribute: StructAttribute) -> Self {
        self.attributes.insert(name.to_string(), attribute);
        self
    }

    pub fn attributes(&self) -> &HashMap<String, StructAttribute> {
        &self.attributes
    }

    pub fn byte_size(&self) -> usize {
        self.attributes
            .values()
            .fold(0, |acc, att| acc + att.data_type.byte_size())
    }
}

impl Default for PayloadDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BufferDescriptor {
    name: String,
    attributes: HashMap<String, StructAttribute>,
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

    pub fn with_attribute(mut self, name: &str, att: StructAttribute) -> Self {
        self.attributes.insert(name.to_string(), att);
        self
    }

    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn attributes(&self) -> &HashMap<String, StructAttribute> {
        &self.attributes
    }

    pub fn byte_size(&self) -> usize {
        self.attributes
            .values()
            .fold(0, |acc, att| acc + att.data_type.byte_size())
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct ImageDescriptor {
    pub name: String,
    pub bits_per_channel: u32,
    pub is_float: bool,
    pub is_read_only: bool,
}

impl ImageDescriptor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            bits_per_channel: 4,
            is_float: false,
            is_read_only: false,
        }
    }

    pub fn with_bits_per_channel(mut self, bits_per_channel: u32) -> Self {
        self.bits_per_channel = bits_per_channel;
        self
    }

    pub fn with_float(mut self, is_float: bool) -> Self {
        self.is_float = is_float;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }
}

pub struct RayTracingPipelineDescriptor {
    pub ray_payload_descriptor: PayloadDescriptor,
    pub ray_generation_source: ShaderSource,
    pub closest_hit_shader_source: ShaderSource,
    pub miss_shader_sources: Vec<ShaderFunction>,
    pub any_hit_shader_source: Option<ShaderSource>,
    pub max_frames_in_flight: u32,
    pub intersection_functions: Vec<ShaderSource>,
    pub buffers: Vec<BufferDescriptor>,
    pub images: Vec<ImageDescriptor>,
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
            images: Vec::new(),
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

    pub fn with_miss_function(mut self, function: ShaderFunction) -> Self {
        self.miss_shader_sources.push(function);
        self
    }

    pub fn with_miss_functions(mut self, functions: Vec<ShaderFunction>) -> Self {
        self.miss_shader_sources.extend(functions);
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

    pub fn with_image_descriptor(mut self, descriptor: ImageDescriptor) -> Self {
        self.images.push(descriptor);
        self
    }
}
