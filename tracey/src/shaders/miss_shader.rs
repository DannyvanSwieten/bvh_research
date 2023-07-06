use libraytracer::{
    cpu::cpu_miss_shader::MissShader,
    types::{HdrColor, Ray, Vec3},
};

use crate::{context::Ctx, payload::Payload};

pub struct MyMissShader;
impl MissShader<Ctx, Payload> for MyMissShader {
    fn execute(&self, _: &Ctx, payload: &mut Payload, ray: &Ray) {
        let d = 0.5 * (ray.direction.y + 1.0);
        let c = (1.0 - d) * Vec3::new(1.0, 1.0, 1.0) + d * Vec3::new(0.5, 0.7, 1.0);
        payload.color = HdrColor::new(c.x, c.y, c.z, 1.0);
    }
}
