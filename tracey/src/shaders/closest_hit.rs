use libraytracer::{cpu::cpu_ray_shader::ClosestHitShader, types::HitRecord};

use crate::{context::Ctx, payload::Payload};

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
        if hit_record.ray_depth == 0 {
            payload.albedo += material.albedo(attributes);
        }
    }
}
