use gpu_tracer::ray_tracer::shader_compiler::ShaderCompiler;

pub fn main() {
    let ray_generation_path = std::env::current_dir()
        .unwrap()
        .join("./example_shaders/ray_gen.glsl");

    let shader_compiler = ShaderCompiler::from_path(&ray_generation_path);
    println!("{}", shader_compiler.compile().source());
}
