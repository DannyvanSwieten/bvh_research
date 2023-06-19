use crate::types::HitRecord;

pub trait MissShader<Context, Payload> {
    fn execute(&self, ctx: &Context, payload: &mut Payload, record: &HitRecord);
}
