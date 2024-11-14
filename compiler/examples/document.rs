use std::rc::Rc;

use compiler::{
    code_generator::{CodeGenerator, GeneratorContext},
    compiler::Compiler,
    document,
    function::{Function, FunctionOverload},
    glsl_compiler::GlslCompiler,
    instruction::{Instruction, Location, Opcode},
};
use intersect::types::DataType;

pub struct ThreadIdCodeGenerator;
impl CodeGenerator for ThreadIdCodeGenerator {
    fn output(
        &self,
        _: usize,
        _: &FunctionOverload,
        ctx: &mut GeneratorContext,
    ) -> Vec<Instruction> {
        vec![Instruction {
            opcode: Opcode::Return,
            operands: vec![Location::Label("thread_id".to_string())],
            result: Some(ctx.next_free_register()),
        }]
    }
}

pub fn main() {
    let function = Function::new("thread_id")
        .with_overload(FunctionOverload::new().with_return_type(DataType::Vec2));

    let generator = ThreadIdCodeGenerator;
    let mut ctx = GeneratorContext::new();

    let mut document = document::Document::new();
    document.add_node("Thread ID", Rc::new(function), Rc::new(generator), 0);

    let glsl_compiler = GlslCompiler {};
    let result = glsl_compiler.compile(&mut ctx, &document);
    println!("{}", result);
}
