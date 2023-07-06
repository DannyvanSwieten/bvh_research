use std::{rc::Rc, time::Instant};

use gpu_tracer::{
    cpu::{
        bvh::BottomLevelAccelerationStructure,
        camera::Camera,
        cpu_miss_shader::MissShader,
        cpu_ray_generator::RayGenerationShader,
        cpu_ray_shader::ClosestHitShader,
        cpu_shader_binding_table::ShaderBindingTable,
        shape::{Shape, SurfaceAttributes},
        top_level_acceleration_structure::{Instance, TopLevelAccelerationStructure},
        trace::{CpuTracer, Tracer},
    },
    types::{
        Direction, HdrColor, HitRecord, Mat4, Position, Ray, RayType, Vec2, Vec3, Vec4, Vertex,
        AABB,
    },
    write_hdr_buffer_to_file,
};

use rand::random;

#[link(name = "Denoiser")]
extern "C" {
    fn denoiser_init();
    fn denoiser_denoise(
        width: u32,
        height: u32,
        input: *const f32,
        albedo: *const f32, // optional
        normal: *const f32, // optional
        output: *mut f32,
    );
}

pub struct Denoiser {
    width: u32,
    height: u32,
}

impl Denoiser {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe {
            denoiser_init();
        }

        Self { width, height }
    }

    pub fn denoise(&self, beauty: &[f32], _albedo: &[f32], _normal: &[f32]) -> Vec<f32> {
        unsafe {
            let mut output = vec![0.0; (self.width * self.height * 3) as usize];
            denoiser_denoise(
                self.width,
                self.height,
                beauty.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                output.as_mut_ptr(),
            );
            output
        }
    }

    pub fn denoise_hdr(&self, beauty: &[HdrColor], _albedo: &[f32], _normal: &[f32]) -> Vec<f32> {
        let beauty = beauty
            .iter()
            .map(|c| Vec3::new(c.x, c.y, c.z))
            .collect::<Vec<Vec3>>();
        let mut output = vec![0.0; (self.width * self.height * 3) as usize];
        unsafe {
            denoiser_denoise(
                self.width,
                self.height,
                beauty.as_ptr() as _,
                std::ptr::null(),
                std::ptr::null(),
                output.as_mut_ptr(),
            );

            output
        }
    }
}

pub trait ImageSampler {
    fn sample_1d(&self, image: &ImageTexture, location: Vec2) -> HdrColor;
    fn sample_2d(&self, image: &ImageTexture, location: Vec2) -> HdrColor;
    fn sample_3d(&self, image: &ImageTexture, location: Vec2) -> HdrColor;
}

pub struct NearestNeighborSampler {}

impl ImageSampler for NearestNeighborSampler {
    fn sample_1d(&self, image: &ImageTexture, location: Vec2) -> HdrColor {
        let x = (location.x * image.image.width() as f32) as u32;
        let pixel = image.image.get_pixel(x, 0);
        HdrColor::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }

    fn sample_2d(&self, image: &ImageTexture, location: Vec2) -> HdrColor {
        let x = (location.x * image.width() as f32) as u32;
        let y = (location.y * image.height() as f32) as u32;
        let pixel = image.image.get_pixel(x, y);
        HdrColor::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }

    fn sample_3d(&self, image: &ImageTexture, location: Vec2) -> HdrColor {
        let x = (location.x * image.width() as f32) as u32;
        let y = (location.y * image.height() as f32) as u32;
        let pixel = image.image.get_pixel(x, y);
        HdrColor::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }
}

pub trait Texture {
    fn sample(&self, attributes: &SurfaceAttributes) -> HdrColor;
}

pub struct UniformColorTexture {
    pub color: HdrColor,
}

impl UniformColorTexture {
    pub fn new(color: HdrColor) -> Self {
        Self { color }
    }
}

impl Texture for UniformColorTexture {
    fn sample(&self, _attributes: &SurfaceAttributes) -> HdrColor {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
    pub scale: f32,
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>, scale: f32) -> Self {
        Self { odd, even, scale }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, attributes: &SurfaceAttributes) -> HdrColor {
        let sines =
            (self.scale * attributes.position.x).sin() * (self.scale * attributes.position.y).sin();
        if sines < 0.0 {
            self.odd.sample(attributes)
        } else {
            self.even.sample(attributes)
        }
    }
}

pub struct NoiseTexture<NoiseGenerator: noise::NoiseFn<f64, 3>> {
    pub scale: f64,
    pub noise_generator: NoiseGenerator,
}

impl<NoiseGenerator: noise::NoiseFn<f64, 3>> NoiseTexture<NoiseGenerator> {
    pub fn new(scale: f64, noise_generator: NoiseGenerator) -> Self {
        Self {
            scale,
            noise_generator,
        }
    }
}

impl<NoiseGenerator: noise::NoiseFn<f64, 3>> Texture for NoiseTexture<NoiseGenerator> {
    fn sample(&self, attributes: &SurfaceAttributes) -> HdrColor {
        let n = self.noise_generator.get([
            self.scale * attributes.position.x as f64,
            self.scale * attributes.position.y as f64,
            self.scale * attributes.position.z as f64,
        ]) as f32;

        HdrColor::new(n, n, n, 1.0)
    }
}

pub struct ImageTexture {
    pub image: image::Rgba32FImage,
}

impl ImageTexture {
    pub fn new(image: image::Rgba32FImage) -> Self {
        Self { image }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}
pub trait Material {
    fn scatter(&self, sampler: &dyn Sampler, attributes: &SurfaceAttributes) -> Direction;
    fn bsdf(&self, attributes: &SurfaceAttributes, direction: &Direction) -> HdrColor;
    fn emission(&self, _attributes: &SurfaceAttributes) -> HdrColor {
        HdrColor::new(0.0, 0.0, 0.0, 1.0)
    }
}

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, sampler: &dyn Sampler, attributes: &SurfaceAttributes) -> Direction {
        let onb = OrthoNormalBasis::new(&attributes.normal);
        onb.to_local(&sampler.sample_hemisphere())
    }

    fn bsdf(&self, attributes: &SurfaceAttributes, _direction: &Direction) -> HdrColor {
        self.albedo.sample(attributes)
    }
}

pub struct Mirror {
    pub albedo: Rc<dyn Texture>,
}

impl Mirror {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Mirror {
    fn scatter(&self, _sampler: &dyn Sampler, _attributes: &SurfaceAttributes) -> Direction {
        //reflect(&Direction::new(1.0, 1.0, 1.0), &attributes.normal)
        Direction::new(1.0, 1.0, 1.0)
    }

    fn bsdf(&self, attributes: &SurfaceAttributes, _direction: &Direction) -> HdrColor {
        self.albedo.sample(attributes)
    }
}

pub struct Emissive {
    pub albedo: Rc<dyn Texture>,
}

impl Emissive {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Emissive {
    fn scatter(&self, sampler: &dyn Sampler, attributes: &SurfaceAttributes) -> Direction {
        let onb = OrthoNormalBasis::new(&attributes.normal);
        onb.to_local(&sampler.sample_hemisphere())
    }

    fn bsdf(&self, _attributes: &SurfaceAttributes, _direction: &Direction) -> HdrColor {
        HdrColor::new(0.0, 0.0, 0.0, 1.0)
    }

    fn emission(&self, attributes: &SurfaceAttributes) -> HdrColor {
        self.albedo.sample(attributes)
    }
}

pub struct SphereShape {
    pub radius: f32,
}

impl SphereShape {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Shape for SphereShape {
    fn intersect(
        &self,
        ray: &gpu_tracer::types::Ray,
        _ray_type: RayType,
        transform: &Mat4,
        t_max: f32,
    ) -> Option<HitRecord> {
        // sphere intersection
        let center = transform * Vec4::new(0.0, 0.0, 0.0, 1.0);
        let l = ray.origin - center.xyz();
        let a = ray.direction.dot(&ray.direction);
        let b = l.dot(&ray.direction) * 2.0;
        let c = l.dot(&l) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        let mut x0;
        let mut x1;
        if discriminant < 0.0 {
            return None;
        } else if discriminant == 0.0 {
            let v = -0.5 * b / a;
            x0 = v;
            x1 = v;
        } else {
            let q = if b > 0.0 {
                -0.5 * (b + discriminant.sqrt())
            } else {
                -0.5 * (b - discriminant.sqrt())
            };
            x0 = q / a;
            x1 = c / q;
        }

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
        }

        if x0 < 0.0 {
            x0 = x1;
            if x0 < 0.0 {
                return None;
            }
        }

        if x0 > t_max {
            return None;
        }

        let hit_record = HitRecord {
            t: x0,
            ray: *ray,
            ..Default::default()
        };
        Some(hit_record)
    }

    fn aabb(&self) -> AABB {
        AABB::new(
            Vec3::new(-self.radius, -self.radius, -self.radius),
            Vec3::new(self.radius, self.radius, self.radius),
        )
    }

    fn surface_attributes(
        &self,
        hit_record: &HitRecord,
    ) -> gpu_tracer::cpu::shape::SurfaceAttributes {
        let position = hit_record.ray.origin + hit_record.ray.direction * hit_record.t;
        let origin = hit_record.obj_to_world * Vec4::new(0.0, 0.0, 0.0, 1.0);
        let normal = (position - origin.xyz()).normalize();
        let u = 0.5 + normal.x.atan2(normal.z) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - normal.y.asin() / std::f32::consts::PI;
        let uv = Vec2::new(u, v);
        SurfaceAttributes {
            position,
            normal,
            uv,
        }
    }
}

pub struct TriangleMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub blas: Rc<BottomLevelAccelerationStructure>,
}

impl Shape for TriangleMesh {
    fn intersect(
        &self,
        ray: &gpu_tracer::types::Ray,
        ray_type: RayType,
        transform: &Mat4,
        t_max: f32,
    ) -> Option<HitRecord> {
        self.blas.traverse(ray, ray_type, transform, t_max)
    }

    fn aabb(&self) -> AABB {
        *self.blas.aabb()
    }

    fn surface_attributes(&self, hit_record: &HitRecord) -> SurfaceAttributes {
        let i = self.indices[hit_record.primitive_id as usize] as usize;
        let i0 = i;
        let i1 = i + 1;
        let i2 = i + 2;
        let v0 = self.vertices[i0];
        let v1 = self.vertices[i1];
        let v2 = self.vertices[i2];
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(&e2).normalize();

        SurfaceAttributes {
            position: hit_record.ray.origin + hit_record.ray.direction * hit_record.t,
            normal,
            uv: Vec2::new(0.0, 0.0),
        }
    }
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
    pub shapes: Vec<Rc<dyn Shape>>,
    pub instances: Vec<Instance>,
    pub materials: Vec<Rc<dyn Material>>,
    pub instance_to_mesh: Vec<usize>,
    pub instance_to_material: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            instances: Vec::new(),
            materials: vec![Rc::new(Lambertian::new(Rc::new(UniformColorTexture::new(
                HdrColor::new(0.5, 0.5, 0.5, 1.0),
            ))))],
            instance_to_mesh: Vec::new(),
            instance_to_material: Vec::new(),
        }
    }

    pub fn build(&mut self) -> TopLevelAccelerationStructure {
        TopLevelAccelerationStructure::new(&self.instances)
    }

    pub fn add_shape(&mut self, shape: Rc<dyn Shape>) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn create_instance(&mut self, object_id: usize, transform: Mat4) -> usize {
        let instance_id = self.instances.len();
        let instance = Instance::new(
            self.shapes[object_id].clone(),
            self.instances.len() as u32,
            transform,
        );
        self.instances.push(instance);
        self.instance_to_mesh.push(object_id);
        self.instance_to_material.push(0);
        instance_id
    }

    fn shape_id_from_instance(&self, instance_id: usize) -> usize {
        self.instance_to_mesh[instance_id]
    }

    pub fn shape_from_instance(&self, instance_id: usize) -> &dyn Shape {
        let shape_id = self.shape_id_from_instance(instance_id);
        self.shape(shape_id)
    }

    pub fn shape(&self, shape_id: usize) -> &dyn Shape {
        self.shapes[shape_id].as_ref()
    }

    pub fn add_material(&mut self, material: Rc<dyn Material>) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn set_material(&mut self, instance_id: usize, material_id: usize) {
        self.instance_to_material[instance_id] = material_id;
    }

    pub fn material(&self, instance_id: usize) -> &dyn Material {
        let material_id = self.instance_to_material[instance_id];
        self.materials[material_id].as_ref()
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
    pub emission: HdrColor,
    pub normal: Direction,
    pub next_direction: Direction,
    pub p: Position,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            color: HdrColor::new(0.0, 0.0, 0.0, 0.0),
            emission: HdrColor::new(0.0, 0.0, 0.0, 0.0),
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
            for i in 0..32 {
                let record = tlas.trace(ctx, sbt, &ray, RayType::Primary, payload, 0);
                factor.x *= payload.color.x;
                factor.y *= payload.color.y;
                factor.z *= payload.color.z;
                factor.x += payload.emission.x;
                factor.y += payload.emission.y;
                factor.z += payload.emission.z;

                if i == 0 {
                    output_color.x += payload.emission.x;
                    output_color.y += payload.emission.y;
                    output_color.z += payload.emission.z;
                }

                if record.is_none() {
                    break;
                }

                ray.origin = payload.p + payload.normal * 0.001;
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
        let attributes = &ctx
            .scene
            .shape_from_instance(hit_record.object_id as usize)
            .surface_attributes(hit_record);

        payload.p = hit_record.ray.origin + hit_record.ray.direction * hit_record.t;
        payload.normal = attributes.normal;

        let material = ctx.scene.material(hit_record.object_id as usize);
        payload.next_direction = material.scatter(ctx.sampler.as_ref(), attributes);
        payload.color = material.bsdf(attributes, &hit_record.ray.direction);
        payload.emission = material.emission(attributes);
    }
}

pub struct MyMissShader;
impl MissShader<Ctx, Payload> for MyMissShader {
    fn execute(&self, _: &Ctx, payload: &mut Payload, ray: &Ray) {
        let d = 0.5 * (ray.direction.y + 1.0);
        let c = (1.0 - d) * Vec3::new(1.0, 1.0, 1.0) + d * Vec3::new(0.5, 0.7, 1.0);
        payload.color = HdrColor::new(c.x, c.y, c.z, 1.0)
    }
}

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
        UniformColorTexture::new(HdrColor::new(10.0, 0.0, 0.0, 1.0)),
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

    let width = 640;
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
        spp: 32,
        camera,
        sampler: Box::new(RandomSampler {}),
        scene,
    };
    let now = Instant::now();
    tracer.trace(&ctx, width, height, &sbt, &tlas, &mut result_buffer);
    let elapsed_time = now.elapsed();
    println!("Tracing CPU took {} millis.", elapsed_time.as_millis());

    let mut hdr_buffer: Vec<HdrColor> = result_buffer.into_iter().map(|p| p.color).collect();
    write_hdr_buffer_to_file(
        "cpu_output.png",
        ctx.spp,
        &hdr_buffer,
        width as usize,
        height as usize,
    );

    let denoiser = Denoiser::new(width, height);
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
        "cpu_output_denoised.png",
        ctx.spp,
        &hdr_buffer,
        width as usize,
        height as usize,
    );
}
