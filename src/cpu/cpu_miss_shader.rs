use crate::types::Ray;

pub trait MissShader<Context, Payload> {
    fn execute(&self, ctx: &Context, payload: &mut Payload, ray: &Ray);
}
