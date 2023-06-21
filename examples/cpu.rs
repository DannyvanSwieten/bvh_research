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
    types::{Direction, HdrColor, HitRecord, Mat4, Position, RayType, Vec2, Vec3, Vertex, AABB},
    write_hdr_buffer_to_file,
};

use nalgebra_glm::dot;
use rand::random;

pub struct TriangleMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub blas: Rc<BottomLevelAccelerationStructure>,
}

impl TriangleMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let blas = Rc::new(BottomLevelAccelerationStructure::new(&vertices, &indices));
        Self {
            vertices,
            indices,
            blas,
        }
    }

    pub fn center(&mut self) {
        let mut bb = AABB::default();
        for vertex in &self.vertices {
            bb.grow_with_position(vertex)
        }

        let center = bb.centroid();
        self.vertices = self
            .vertices
            .iter()
            .map(|position| center - position)
            .collect();
    }
}

#[derive(Default)]
pub struct Scene {
    pub meshes: Vec<TriangleMesh>,
    pub instances: Vec<Instance>,
    pub instance_to_mesh: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&mut self) -> TopLevelAccelerationStructure {
        TopLevelAccelerationStructure::new(&self.instances)
    }

    pub fn add_mesh(&mut self, mesh: TriangleMesh) -> usize {
        self.meshes.push(mesh);
        self.meshes.len() - 1
    }

    pub fn create_instance(&mut self, object_id: usize, transform: Mat4) -> usize {
        let instance_id = self.instances.len();
        let instance = Instance::new(
            self.meshes[object_id].blas.clone(),
            self.instances.len() as u32,
            transform,
        );
        self.instances.push(instance);
        self.instance_to_mesh.push(object_id);
        instance_id
    }

    fn mesh_id_from_instance(&self, instance_id: usize) -> usize {
        self.instance_to_mesh[instance_id]
    }

    pub fn mesh_from_instance(&self, instance_id: usize) -> &TriangleMesh {
        let mesh_id = self.mesh_id_from_instance(instance_id);
        self.mesh(mesh_id)
    }

    pub fn mesh(&self, mesh_id: usize) -> &TriangleMesh {
        &self.meshes[mesh_id]
    }
}

pub struct OrthoNormalBasis {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthoNormalBasis {
    pub fn new(n: &Direction) -> Self {
        let w = *n;
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        Self { u, v, w }
    }

    pub fn to_local(&self, v: &Direction) -> Direction {
        self.u * v.x + self.v * v.y + self.w * v.z
    }
}

pub trait Sampler {
    fn sample(&self) -> f32;
    fn sample2(&self) -> Vec2;
    fn sample3(&self) -> Vec3;
    fn sample_hemisphere(&self) -> Vec3;
    fn sample_sphere(&self) -> Vec3;
}

pub struct RandomSampler {}
impl Sampler for RandomSampler {
    fn sample(&self) -> f32 {
        random::<f32>()
    }

    fn sample2(&self) -> Vec2 {
        Vec2::new(random::<f32>(), random::<f32>())
    }

    fn sample3(&self) -> Vec3 {
        Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
    }

    fn sample_hemisphere(&self) -> Vec3 {
        let r1 = self.sample();
        let r2 = self.sample();
        let z = (1.0 - r2).sqrt();
        let phi = std::f32::consts::TAU * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        Vec3::new(x, y, z)
    }

    fn sample_sphere(&self) -> Vec3 {
        loop {
            let p = self.sample3() * 2.0 - Vec3::new(1.0, 1.0, 1.0);
            if nalgebra_glm::length2(&p) < 1.0 {
                return p;
            }
        }
    }
}

pub struct Ctx {
    pub camera: Camera,
    pub scene: Scene,
    pub sampler: Box<dyn Sampler>,
    pub spp: usize,
}

unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}

#[derive(Clone, Copy, Debug)]
pub struct Payload {
    pub color: HdrColor,
    pub normal: Direction,
    pub next_direction: Direction,
    pub p: Position,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            color: HdrColor::new(0.0, 0.0, 0.0, 0.0),
            next_direction: Direction::new(0.0, 0.0, 0.0),
            normal: Direction::new(0.0, 0.0, 0.0),
            p: Position::new(0.0, 0.0, 0.0),
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
        let mut output_color = HdrColor::new(0.0, 0.0, 0.0, 1.0);
        for _ in 0..ctx.spp {
            let mut factor = Vec3::new(1.0, 1.0, 1.0);
            let location = pixel + ctx.sampler.sample2();
            let mut ray = ctx.camera.ray(&location, resolution);
            for _ in 0..16 {
                let record = tlas.trace(ctx, sbt, &ray, RayType::Primary, payload, 0);
                factor.x *= payload.color.x;
                factor.y *= payload.color.y;
                factor.z *= payload.color.z;

                if !record.hit() {
                    break;
                }

                ray.origin = payload.p; // + payload.normal * 0.01;
                ray.direction = payload.next_direction;
            }

            output_color.x += factor.x;
            output_color.y += factor.y;
            output_color.z += factor.z;
        }

        payload.color = output_color;
    }
}

pub struct MyClosestHitShader;
impl ClosestHitShader<Ctx, Payload> for MyClosestHitShader {
    fn execute(&self, ctx: &Ctx, payload: &mut Payload, hit_record: &HitRecord) {
        let vertices = &ctx
            .scene
            .mesh_from_instance(hit_record.object_id as usize)
            .vertices;
        let indices = &ctx
            .scene
            .mesh_from_instance(hit_record.object_id as usize)
            .indices;
        let i = indices[hit_record.primitive_id as usize] as usize;
        let i0 = i;
        let i1 = i + 1;
        let i2 = i + 2;
        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(&e2).normalize();
        let onb = OrthoNormalBasis::new(&normal);
        payload.p = hit_record.ray.origin + hit_record.ray.direction * hit_record.t;
        payload.next_direction = onb.to_local(&ctx.sampler.sample_hemisphere()).normalize();

        payload.normal = normal;
        if hit_record.object_id == 0 {
            payload.color = HdrColor::new(0.5, 0.5, 0.5, 1.0);
        } else {
            payload.color = HdrColor::new(1.0, 0.0, 0.0, 1.0);
        }
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
    let mut scene = Scene::new();
    let (vertices, indices) = read_triangle_file("unity.tri");
    let mut triangle_mesh = TriangleMesh::new(vertices, indices);
    triangle_mesh.center();
    let mesh = scene.add_mesh(triangle_mesh);
    scene.create_instance(mesh, nalgebra_glm::scaling(&Vec3::new(1.0, 1.0, 1.0)));

    let floor_vertices = vec![
        Vertex::new(-1.0, -15.0, -1.0),
        Vertex::new(-1.0, -15.0, 1.0),
        Vertex::new(1.0, -15.0, 1.0),
        Vertex::new(1.0, -15.0, -1.0),
    ];

    let floor_indices = vec![0, 1, 2, 0, 2, 3];

    let mesh = scene.add_mesh(TriangleMesh::new(floor_vertices, floor_indices));
    scene.create_instance(mesh, nalgebra_glm::scaling(&Vec3::new(100.0, 1.0, 100.0)));

    let width = 640;
    let height = 420;
    let camera = Camera::new(
        Position::new(0.0, 0.0, -3.0),
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
        spp: 16,
        camera,
        sampler: Box::new(RandomSampler {}),
        scene,
    };
    let now = Instant::now();
    tracer.trace(&ctx, width, height, &sbt, &tlas, &mut result_buffer);
    let elapsed_time = now.elapsed();
    println!("Tracing CPU took {} millis.", elapsed_time.as_millis());

    let hdr_buffer: Vec<HdrColor> = result_buffer.into_iter().map(|p| p.color).collect();
    write_hdr_buffer_to_file(
        "cpu_output.png",
        ctx.spp,
        &hdr_buffer,
        width as usize,
        height as usize,
    );
}
