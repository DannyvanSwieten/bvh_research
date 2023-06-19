use std::{rc::Rc, time::Instant};

use gpu_tracer::{
    cpu::{
        bvh::BottomLevelAccelerationStructure,
        camera::Camera,
        cpu_miss_shader::MissShader,
        cpu_ray_generator::RayGenerationShader,
        cpu_ray_shader::ClosestHitShader,
        cpu_shader_binding_table::ShaderBindingTable,
        top_level_acceleration_structure::{Instance, TopLevelAccelerationStructure},
        trace::{CpuTracer, Tracer},
    },
    read_triangle_file,
    types::{Direction, HdrColor, HitRecord, Mat4, Position, RayType, Vec2, Vec3},
    write_hdr_buffer_to_file,
};

pub struct Ctx {
    pub camera: Camera,
    pub vertices: Vec<Position>,
    pub indices: Vec<u32>,
}
#[derive(Clone, Copy, Debug)]
pub struct Payload {
    pub color: HdrColor,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            color: HdrColor::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}
pub struct MyRayGenerator;
impl RayGenerationShader<Ctx, Payload> for MyRayGenerator {
    fn execute(
        &self,
        ctx: &Ctx,
        tlas: &TopLevelAccelerationStructure,
        sbt: &ShaderBindingTable<Ctx, Payload>,
        payload: &mut Payload,
        pixel: &Vec2,
        resolution: &Vec2,
    ) {
        let ray = ctx.camera.ray(pixel, resolution);

        let _ = tlas.trace(ctx, sbt, &ray, RayType::Primary, payload, 0);
    }
}

pub struct MyClosestHitShader;
impl ClosestHitShader<Ctx, Payload> for MyClosestHitShader {
    fn execute(&self, ctx: &Ctx, payload: &mut Payload, hit_record: &HitRecord) {
        let i = ctx.indices[hit_record.primitive_id as usize] as usize;
        let i0 = i;
        let i1 = i + 1;
        let i2 = i + 2;
        let v0 = ctx.vertices[i0];
        let v1 = ctx.vertices[i1];
        let v2 = ctx.vertices[i2];
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(&e2).normalize();
        let light_dir = Direction::new(1.0, 1.0, -1.0).normalize();
        let radiance = normal.dot(&light_dir).max(0.0);
        let r = (radiance).sqrt();
        payload.color = HdrColor::new(r, r, r, 1.0);
    }
}

pub struct MyMissShader;
impl MissShader<Ctx, Payload> for MyMissShader {
    fn execute(&self, _: &Ctx, payload: &mut Payload, hit_record: &HitRecord) {
        let d = 0.5 * (hit_record.ray.direction.y + 1.0);
        let c = (1.0 - d) * Vec3::new(1.0, 1.0, 1.0) + d * Vec3::new(0.5, 0.7, 1.0);
        payload.color = HdrColor::new(c.x, c.y, c.z, 1.0)
    }
}

fn main() {
    let (vertices, indices) = read_triangle_file("unity.tri");
    let width = 500;
    let height = 500;
    let camera = Camera::new(
        Position::new(-3.0, 0.0, 10.0),
        45.0,
        &Vec2::new(width as f32, height as f32),
    );
    let mut sbt = ShaderBindingTable::new(Box::new(MyRayGenerator {}));
    sbt.add_closest_hit_shader(Box::new(MyClosestHitShader {}));
    sbt.add_miss_shader(Box::new(MyMissShader {}));
    let blas = Rc::new(BottomLevelAccelerationStructure::new(&vertices, &indices));
    let instances = [Instance::new(
        blas,
        0,
        nalgebra_glm::scaling(&Vec3::new(0.25, 0.25, 0.25)),
    )];
    let tlas = TopLevelAccelerationStructure::new(&instances);
    let tracer = CpuTracer {};

    let mut result_buffer = vec![Payload::default(); (width * height) as usize];
    let ctx = Ctx {
        camera,
        vertices,
        indices,
    };
    let now = Instant::now();
    tracer.trace(&ctx, width, height, &sbt, &tlas, &mut result_buffer);
    let elapsed_time = now.elapsed();
    println!("Tracing CPU took {} millis.", elapsed_time.as_millis());

    let hdr_buffer: Vec<HdrColor> = result_buffer.into_iter().map(|p| p.color).collect();
    write_hdr_buffer_to_file(
        "cpu_output.png",
        1,
        &hdr_buffer,
        width as usize,
        height as usize,
    );
}
