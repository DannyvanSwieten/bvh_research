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


