use compiler::{
    code_generator::{CodeGenerator, GeneratorContext},
    function::{Function, FunctionOverload},
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
            opcode: Opcode::Load,
            operands: vec![Location::Label("thread_id".to_string())],
            result: Some(ctx.next_free_register()),
        }]
    }
}

pub fn main() {
    let function = Function::new("Thread ID")
        .with_overload(FunctionOverload::new().with_return_type(DataType::Vec2));

    let generator = ThreadIdCodeGenerator;
    let mut ctx = GeneratorContext::new();

    let instructions = generator.output(0, &function.overloads[0], &mut ctx);
    for instruction in instructions {
        println!("{:?}", instruction);
    }
}
