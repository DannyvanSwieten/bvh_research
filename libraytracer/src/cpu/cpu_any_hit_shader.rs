use crate::types::HitRecord;

pub trait AnyHitShader<Context, Payload> {
    fn execute(&self, ctx: &Context, payload: &mut Payload, record: &HitRecord);
}
