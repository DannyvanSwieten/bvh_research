use libraytracer::{
    cpu::{
        cpu_ray_generator::RayGenerationShader, cpu_shader_binding_table::ShaderBindingTable,
        top_level_acceleration_structure::TopLevelAccelerationStructure,
    },
    types::{HdrColor, RayType, Vec2, Vec3},
};

use crate::{context::Ctx, payload::Payload};

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
                let record = tlas.trace(ctx, sbt, &ray, RayType::Primary, payload, 0, i);
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

                    payload.first_normal += payload.normal;
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
