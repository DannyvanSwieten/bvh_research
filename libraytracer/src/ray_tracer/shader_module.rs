use std::path::Path;

pub struct ShaderModule {
    pub name: String,
    source: String,
}

impl ShaderModule {
    pub fn new(name: &str, source: &str) -> Self {
        Self {
            name: name.to_string(),
            source: source.to_string(),
        }
    }

    pub fn new_path(path: &Path) -> Self {
        let source = std::fs::read_to_string(path).expect("Couldn't load Shader Module file");
        Self {
            name: path.to_str().unwrap().to_string(),
            source: source.to_string(),
        }
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }
}
