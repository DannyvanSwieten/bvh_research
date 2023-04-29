use gpu_tracer::ray_tracer::{
    shader::{DiffuseShader, MirrorShader},
    shader_compiler::ShaderCompiler,
};

pub fn main() {
    let random_module = std::env::current_dir()
        .unwrap()
        .join("./example_shaders/ray_gen.glsl");

    let shader_compiler = ShaderCompiler::from_path(&random_module);
    println!("{}", shader_compiler.compile());
}
