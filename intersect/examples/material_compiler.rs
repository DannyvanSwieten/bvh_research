use intersect::material::{
    material_compiler::MaterialCompiler,
    shader::{DiffuseShader, MirrorShader},
};

pub fn main() {
    let random_module = std::env::current_dir()
        .unwrap()
        .join("./example_shaders/random.glsl");
    let mut material_compiler = MaterialCompiler::new();
    material_compiler.add_shader(Box::new(DiffuseShader::new()));
    material_compiler.add_shader(Box::new(MirrorShader::new()));
    material_compiler.include(&random_module);
    let src = material_compiler.compile();
    println!("{}", src)
}
