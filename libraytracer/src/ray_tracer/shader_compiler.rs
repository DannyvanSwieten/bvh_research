use std::path::Path;

use crate::ray_tracer::shader_module::ShaderModule;

use super::shader_binding_table::ShaderBindingTable;

pub struct ShaderCompiler {
    modules: Vec<ShaderModule>,
}

impl ShaderCompiler {
    pub fn from_path(path: &Path) -> Self {
        let mut this = Self {
            modules: Vec::new(),
        };

        this.add_file(path);
        this
    }

    pub fn preprocess_file(&mut self, path: &Path) -> String {
        let mut src = std::fs::read_to_string(path).unwrap();
        let mut begin_idx = 0;
        let mut includes = Vec::new();
        loop {
            if let Some(include_index) = src[begin_idx..].find("#include") {
                if let Some(end_of_include_index) = src[include_index..].find('\n') {
                    let end_idx = include_index + end_of_include_index;
                    let include = &src[include_index..end_idx];
                    let include_path = include.split('"').nth(1).unwrap();
                    let include_full_path = path.parent().unwrap().join(include_path);
                    includes.push(include.to_string());
                    self.add_file(&include_full_path);
                    begin_idx = end_idx + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        includes.into_iter().for_each(|include| {
            src = src.replace(include.as_str(), "");
        });

        src
    }

    fn add_file(&mut self, path: &Path) {
        let src = self.preprocess_file(path);
        let module = ShaderModule::new(path.file_name().unwrap().to_str().unwrap(), &src);
        self.add_module(module);
    }

    pub fn include(&mut self, path: &Path) {
        self.add_module(ShaderModule::new_path(path))
    }

    fn add_module(&mut self, module: ShaderModule) {
        self.modules.push(module);
    }

    fn output_hit_shaders(&self, sbt: &ShaderBindingTable) -> String {
        let mut result = String::new();
        for shader in sbt.ray_hit_shaders() {
            result.push_str(shader.source());
            result.push('\n');
        }

        result
    }

    fn output_library_functions(&self) -> String {
        let mut result = String::new();
        for module in &self.modules {
            result.push_str(module.source());
            result.push('\n');
        }
        result
    }

    pub fn compile(&self) -> ShaderModule {
        let mut result = String::new();
        for module in &self.modules {
            result.push_str(module.source());
            result.push('\n');
        }

        ShaderModule::new("name", &result)
    }
}
