use std::{rc::Rc, time::Instant};

use gpu_tracer::{
    bvh::Bvh,
    camera::Camera,
    cpu::{
        cpu_ray_generator::RayGenerationShader, cpu_shader_binding_table::ShaderBindingTable,
        frame_buffer::Framebuffer,
    },
    read_triangle_file,
    top_level_acceleration_structure::{Instance, TopLevelAccelerationStructure},
    trace::{CpuTracer, Tracer},
    types::{HdrColor, Mat4, Position, RayType, Vec2},
    write_framebuffer_to_file,
};

pub struct Ctx<'a> {
    pub camera: Camera,
    pub framebuffer: &'a mut Framebuffer<HdrColor>,
}

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
impl<'a> RayGenerationShader<Ctx<'a>, Payload> for MyRayGenerator {
    fn execute(
        &self,
        ctx: &Ctx<'a>,
        tlas: &TopLevelAccelerationStructure,
        sbt: &ShaderBindingTable<Ctx<'a>, Payload>,
        payload: &mut Payload,
        pixel: Vec2,
        resolution: Vec2,
    ) {
        let ray = ctx.camera.ray(
            pixel.x as usize,
            pixel.y as usize,
            resolution.x as usize,
            resolution.y as usize,
        );

        let hit_record = tlas.trace(ctx, sbt, &ray, RayType::Primary);
        if hit_record.hit() {
            payload.color = HdrColor::new(1.0, 0.0, 0.0, 0.0);
        }
    }
}

fn main() {
    let (vertices, indices) = read_triangle_file("unity.tri");
    let width = 640;
    let height = 640;
    let mut framebuffer = Framebuffer::new(640, 640, HdrColor::new(0.0, 0.0, 0.0, 0.0));
    let camera = Camera::new(Position::new(-5.0, 0.0, -15.0), 2.0);
    let sbt = ShaderBindingTable::new(Box::new(MyRayGenerator {}));

    let midpoint_split_acc = Rc::new(Bvh::new(&vertices, &indices));

    let instances = [Instance::new(midpoint_split_acc, 0, Mat4::from_scale(1.0))];

    let tlas = TopLevelAccelerationStructure::new(&instances);

    let tracer = CpuTracer {};
    {
        let ctx = Ctx {
            camera,
            framebuffer: &mut framebuffer,
        };
        let now = Instant::now();
        tracer.trace(&ctx, width, height, &sbt, &tlas);
        let elapsed_time = now.elapsed();
        println!("Tracing CPU took {} millis.", elapsed_time.as_millis());
    }
    // write_framebuffer_to_file("cpu.png", &framebuffer);
}
