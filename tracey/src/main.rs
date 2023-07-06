use std::{rc::Rc, time::Instant};

use libraytracer::{
    cpu::{
        camera::Camera,
        cpu_shader_binding_table::ShaderBindingTable,
        trace::{CpuTracer, Tracer},
    },
    types::{Direction, HdrColor, Position, Vec2, Vec3, Vertex},
};
use tracey_utils::{
    denoiser::Denoiser,
    material::{Emissive, Lambertian},
    sampler::RandomSampler,
    sphere_shape::SphereShape,
    texture::{CheckerTexture, NoiseTexture, UniformColorTexture},
    triangle_mesh_shape::TriangleMesh,
    write_image::write_hdr_buffer_to_file,
};

use crate::{
    context::Ctx,
    payload::Payload,
    scene::Scene,
    shaders::{
        closest_hit::MyClosestHitShader, miss_shader::MyMissShader, ray_generation::MyRayGenerator,
    },
};

pub mod context;
pub mod payload;
pub mod scene;
pub mod shaders;
fn main() {
    let mut scene = Scene::new();
    let solid_color_material = scene.add_material(Rc::new(Lambertian::new(Rc::new(
        UniformColorTexture::new(HdrColor::new(0.5, 0.5, 0.5, 1.0)),
    ))));
    let checker_material =
        scene.add_material(Rc::new(Lambertian::new(Rc::new(CheckerTexture::new(
            Rc::new(UniformColorTexture::new(HdrColor::new(0.2, 0.3, 0.1, 1.0))),
            Rc::new(UniformColorTexture::new(HdrColor::new(0.9, 0.9, 0.9, 1.0))),
            10.0,
        )))));

    let noise_material = scene.add_material(Rc::new(Lambertian::new(Rc::new(NoiseTexture::new(
        5.0,
        noise::Fbm::<noise::Perlin>::default(),
    )))));

    let emissive_material = scene.add_material(Rc::new(Emissive::new(Rc::new(
        UniformColorTexture::new(HdrColor::new(1.0, 0.0, 0.0, 1.0)),
    ))));

    let floor_vertices = vec![
        Vertex::new(-100.0, -1.0, -100.0),
        Vertex::new(-100.0, -1.0, 100.0),
        Vertex::new(100.0, -1.0, 100.0),
        Vertex::new(100.0, -1.0, -100.0),
    ];

    let floor_indices = vec![0, 1, 2, 0, 2, 3];

    let mesh = scene.add_shape(Rc::new(TriangleMesh::new(floor_vertices, floor_indices)));
    let floor_instance =
        scene.create_instance(mesh, nalgebra_glm::translation(&Vec3::new(0.0, 0.0, 0.0)));

    let sphere = scene.add_shape(Rc::new(SphereShape::new(1.0)));
    let sphere_instance =
        scene.create_instance(sphere, nalgebra_glm::translation(&Vec3::new(0.0, 1.0, 0.0)));

    scene.set_material(sphere_instance, emissive_material);

    scene.set_material(floor_instance, solid_color_material);

    let sphere_instance =
        scene.create_instance(sphere, nalgebra_glm::translation(&Vec3::new(2.0, 0.0, 0.0)));

    scene.set_material(sphere_instance, checker_material);

    let sphere_instance = scene.create_instance(
        sphere,
        nalgebra_glm::translation(&Vec3::new(-2.0, 0.0, 0.0)),
    );

    scene.set_material(sphere_instance, noise_material);

    let width = 720;
    let height = 480;
    let camera = Camera::new(
        Position::new(0.0, 0.0, -5.0),
        65.0,
        &Vec2::new(width as f32, height as f32),
    );
    let mut sbt = ShaderBindingTable::new(Box::new(MyRayGenerator {}));
    sbt.add_closest_hit_shader(Box::new(MyClosestHitShader {}));
    sbt.add_miss_shader(Box::new(MyMissShader {}));

    let tlas = scene.build();
    let tracer = CpuTracer {};

    let mut result_buffer = vec![Payload::default(); (width * height) as usize];
    let ctx = Ctx {
        spp: 8,
        camera,
        sampler: Box::new(RandomSampler {}),
        scene,
    };
    let now = Instant::now();
    tracer.trace(&ctx, width, height, &sbt, &tlas, &mut result_buffer);
    let elapsed_time = now.elapsed();
    println!("Tracing CPU took {} millis.", elapsed_time.as_millis());

    let mut hdr_buffer: Vec<HdrColor> = result_buffer.iter().map(|p| p.color).collect();
    write_hdr_buffer_to_file(
        "outputs/cpu_output.png",
        ctx.spp,
        &hdr_buffer,
        width as usize,
        height as usize,
        true,
        true,
    );

    let normal_hdr_buffer: Vec<HdrColor> = result_buffer
        .iter()
        .map(|p| HdrColor::new(p.first_normal.x, p.first_normal.y, p.first_normal.z, 1.0))
        .collect();
    write_hdr_buffer_to_file(
        "outputs/cpu_output_normals.png",
        ctx.spp,
        &normal_hdr_buffer,
        width as usize,
        height as usize,
        false,
        false,
    );

    let albedo_hdr_buffer: Vec<HdrColor> = result_buffer.iter().map(|p| p.albedo).collect();
    write_hdr_buffer_to_file(
        "outputs/cpu_output_albedo.png",
        ctx.spp,
        &albedo_hdr_buffer,
        width as usize,
        height as usize,
        false,
        false,
    );

    let normal_buffer: Vec<Direction> = result_buffer
        .iter()
        .map(|p| p.first_normal * (1.0 / ctx.spp as f32))
        .collect();
    let albedo_buffer: Vec<Vec3> = result_buffer
        .iter()
        .map(|p| p.albedo.xyz() * (1.0 / ctx.spp as f32))
        .collect();

    let denoiser = Denoiser::new(width, height);
    let denoised_buffer = denoiser.denoise_hdr(&hdr_buffer, &albedo_buffer, &normal_buffer);
    for i in 0..hdr_buffer.len() {
        let r = denoised_buffer[i * 3];
        let g = denoised_buffer[i * 3 + 1];
        let b = denoised_buffer[i * 3 + 2];
        let a = 1.0f32;
        let c = HdrColor::new(r, g, b, a);
        hdr_buffer[i] = c;
    }

    write_hdr_buffer_to_file(
        "outputs/cpu_output_denoised_with_auxiliary.png",
        ctx.spp,
        &hdr_buffer,
        width as usize,
        height as usize,
        true,
        true,
    );

    let denoised_buffer = denoiser.denoise_hdr(&hdr_buffer, &[], &[]);
    for i in 0..hdr_buffer.len() {
        let r = denoised_buffer[i * 3];
        let g = denoised_buffer[i * 3 + 1];
        let b = denoised_buffer[i * 3 + 2];
        let a = 1.0f32;
        let c = HdrColor::new(r, g, b, a);
        hdr_buffer[i] = c;
    }

    write_hdr_buffer_to_file(
        "outputs/cpu_output_denoised_no_auxiliary.png",
        ctx.spp,
        &hdr_buffer,
        width as usize,
        height as usize,
        true,
        true,
    );
}
