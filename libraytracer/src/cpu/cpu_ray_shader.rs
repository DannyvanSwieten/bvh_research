use crate::types::HitRecord;

pub trait ClosestHitShader<Context, Payload> {
    fn execute(&self, ctx: &Context, payload: &mut Payload, record: &HitRecord);
}
