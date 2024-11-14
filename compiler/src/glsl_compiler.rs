use crate::{
    code_generator::GeneratorContext, compiler::Compiler, document::Document,
    instruction::Instruction,
};

pub struct GlslCompiler {}

impl GlslCompiler {
    fn output_definitions(&self, document: &Document) -> String {
        let mut result = String::new();
        document.for_each_node(|_, node| {
            let function = node.function();
            let overload_index = node.overload();
            let overload = &function.overloads()[overload_index];
            result += &format!("{} {}();\n", overload.return_type, function.name());
        });

        result
    }

    fn output_implementations(&self, document: &Document, ctx: &mut GeneratorContext) -> String {
        let mut result = String::new();
        document.for_each_node(|_, node| {
            let function = node.function();
            let overload_index = node.overload();
            let overload = &function.overloads()[overload_index];
            let generator = node.generator();
            let instructions = generator.output(overload_index, overload, ctx);
            result += &format!("{} {}() {{\n", overload.return_type, function.name());

            for instruction in instructions {
                match instruction.opcode {
                    crate::instruction::Opcode::Call => todo!(),
                    crate::instruction::Opcode::Load => todo!(),
                    crate::instruction::Opcode::Add => todo!(),
                    crate::instruction::Opcode::Return => {
                        let result_name = match &instruction.operands[0] {
                            crate::instruction::Location::Register(reg) => format!("r{}", reg),
                            crate::instruction::Location::Memory(mem) => format!("m{}", mem),
                            crate::instruction::Location::Immediate(imm) => format!("{}", imm),
                            crate::instruction::Location::Label(label) => label.to_string(),
                        };
                        result += &format!("    return {};\n", result_name);
                    }
                }
            }
            result += "}\n";
        });

        result
    }

    // fn generate_function_calls(&self, document: &Document) -> Vec<Instruction> {
    //     document.for_each_node(|_, node|{
    //         let mut instructions = Vec::new();
    //         let function = node.function();
    //         let overload_index = node.overload();
    //         let overload = &function.overloads()[overload_index];
    //         instructions.push()
    //     });
    // }
}

impl Compiler for GlslCompiler {
    fn compile(&self, ctx: &mut GeneratorContext, document: &Document) -> String {
        let mut result = String::new();

        result += "#version 460\n";
        result += "// Function Definitions\n";
        result += &self.output_definitions(document);

        result += "// Function Implementations\n";

        result += &self.output_implementations(document, ctx);

        result
    }
}
