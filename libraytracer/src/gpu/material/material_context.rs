use crate::types::{Vec2, Vec3, Vec4};

pub struct MaterialContext {
    float_data: Vec<f32>,
    texture_data: Vec<u32>,
    float_index: u32,
    texture_index: u32,
}

impl MaterialContext {
    pub fn push_texture(&mut self, value: u32) {
        let index = self.texture_data.len() as u32;
        self.texture_data.push(value);
        self.texture_index = index;
    }
    pub fn push_float(&mut self, value: f32) {
        let index = self.float_data.len() as u32;
        self.float_data.push(value);
        self.float_index = index;
    }

    pub fn push_float2(&mut self, value: Vec2) {
        let index = self.float_data.len() as u32 + 1;
        self.float_data.push(value[0]);
        self.float_data.push(value[1]);
        self.float_index = index;
    }

    pub fn push_float3(&mut self, value: Vec3) {
        let index = self.float_data.len() as u32 + 2;
        self.float_data.push(value[0]);
        self.float_data.push(value[1]);
        self.float_data.push(value[2]);
        self.float_index = index;
    }

    pub fn push_float4(&mut self, value: Vec4) {
        let index = self.float_data.len() as u32 + 3;
        self.float_data.push(value[0]);
        self.float_data.push(value[1]);
        self.float_data.push(value[2]);
        self.float_data.push(value[3]);
        self.float_index = index;
    }
}
