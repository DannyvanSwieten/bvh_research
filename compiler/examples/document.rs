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
    fn output(&self, _: usize, _: &FunctionOverload, _: &mut GeneratorContext) -> Vec<Instruction> {
        vec![Instruction {
            opcode: Opcode::Return,
            operands: vec![Location::Label("thread_id".to_string())],
            result: None,
        }]
    }
}

pub struct AddCodeGenerator;
impl CodeGenerator for AddCodeGenerator {
    fn output(
        &self,
        _: usize,
        overload: &FunctionOverload,
        _: &mut GeneratorContext,
    ) -> Vec<Instruction> {
        vec![
            Instruction {
                opcode: Opcode::Add,
                operands: vec![
                    Location::Argument(overload.arguments[0].name.clone()),
                    Location::Argument(overload.arguments[1].name.clone()),
                ],
                result: Some(Location::Label("result".to_string())),
            },
            Instruction {
                opcode: Opcode::Return,
                operands: vec![Location::Label("result".to_string())],
                result: None,
            },
        ]
    }
}

pub fn main() {
    let thread_id_function = Function::new("thread_id")
        .with_overload(FunctionOverload::new().with_return_type(DataType::Vec2));
    let thread_id_function_generator = ThreadIdCodeGenerator;

    let add_function = Function::new("add").with_overload(
        FunctionOverload::new()
            .with_return_type(DataType::Vec2)
            .with_argument("a", DataType::Vec2)
            .with_argument("b", DataType::Vec2),
    );
    let add_function_generator = AddCodeGenerator;

    let mut ctx = GeneratorContext::new();

    let mut document = document::Document::new();
    document.add_node(
        "Thread ID",
        Rc::new(thread_id_function),
        Rc::new(thread_id_function_generator),
        0,
    );

    document.add_node(
        "Add",
        Rc::new(add_function),
        Rc::new(add_function_generator),
        0,
    );

    let glsl_compiler = GlslCompiler {};
    let result = glsl_compiler.compile(&mut ctx, &document);
    println!("{}", result);
}
