use std::{collections::HashMap, path::Path};

use super::{shader::Shader, shader_module::ShaderModule};

pub struct MaterialCompiler {
    modules: HashMap<String, ShaderModule>,
    shaders: Vec<Box<dyn Shader>>,
}

impl MaterialCompiler {
    pub fn new() -> Self {
        Self {
            shaders: Vec::new(),
            modules: HashMap::new(),
        }
    }

    pub fn include(&mut self, path: &Path) {
        self.add_module(ShaderModule::new_path(path))
    }

    pub fn add_module(&mut self, module: ShaderModule) {
        self.modules.insert(module.name.to_string(), module);
    }

    pub fn add_shader(&mut self, shader: Box<dyn Shader>) -> usize {
        self.shaders.push(shader);
        self.shaders.len() - 1
    }

    fn output_hit_shaders(&self) -> String {
        let mut result = String::new();
        for shader in &self.shaders {
            result.push_str(shader.source());
            result.push('\n');
        }

        result
    }

    fn output_library_functions(&self) -> String {
        let mut result = String::new();
        self.modules.iter().for_each(|(_, module)| {
            result.push_str(module.source());
            result.push('\n');
        });
        result
    }

    pub fn compile(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.output_library_functions());
        result.push_str(&self.output_hit_shaders());
        for shader in &self.shaders {
            result.push_str(
                std::format!(
                    "if (material.shader_id == {}) {{
                        return {} (parameter_offset, ray, instance_id, primitive_id);
                    }}",
                    shader.uid(),
                    shader.name()
                )
                .as_str(),
            );
            result.push('\n')
        }
        result
    }
}

impl Default for MaterialCompiler {
    fn default() -> Self {
        Self::new()
    }
}
