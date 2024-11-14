use crate::{code_generator::GeneratorContext, document::Document};

pub trait Compiler {
    fn compile(&self, ctx: &mut GeneratorContext, document: &Document) -> String;
}
