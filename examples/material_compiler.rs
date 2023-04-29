use gpu_tracer::ray_tracer::{
    shader::{DiffuseShader, MirrorShader},
    shader_compiler::ShaderCompiler,
};

pub fn main() {
    let random_module = std::env::current_dir()
        .unwrap()
        .join("./example_shaders/ray_gen.glsl");
    // let mut material_compiler = MaterialCompiler::new();
    // material_compiler.add_shader(Box::new(DiffuseShader::new()));
    // material_compiler.add_shader(Box::new(MirrorShader::new()));
    // material_compiler.include(&random_module);
    // let src = material_compiler.compile();
    // println!("{}", src);

    let material_compiler = ShaderCompiler::from_path(&random_module);
    println!("{}", material_compiler.compile());
}
