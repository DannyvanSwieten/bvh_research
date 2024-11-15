use crate::{
    code_generator::GeneratorContext, compiler::Compiler, document::Document, function::Argument,
    instruction::Instruction,
};

pub struct GlslCompiler {}

impl GlslCompiler {
    fn construct_args(&self, args: &[Argument]) -> String {
        args.iter()
            .enumerate()
            .fold(String::new(), |acc, (i, arg)| {
                if i < args.len() - 1 {
                    acc + &format!("{} {}, ", arg.data_type, arg.name)
                } else {
                    acc + &format!("{} {}", arg.data_type, arg.name)
                }
            })
    }

    fn output_definitions(&self, document: &Document) -> String {
        let mut result = String::new();
        document.for_each_node(|_, node| {
            let function = node.function();
            let overload_index = node.overload();
            let overload = &function.overloads()[overload_index];
            let args = self.construct_args(&overload.arguments);
            result += &format!("{} {}({});\n", overload.return_type, function.name(), args);
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
            let args = self.construct_args(&overload.arguments);
            result += &format!(
                "{} {}({}) {{\n",
                overload.return_type,
                function.name(),
                args
            );

            for instruction in instructions {
                match instruction.opcode {
                    crate::instruction::Opcode::Call => todo!(),
                    crate::instruction::Opcode::Load => todo!(),
                    crate::instruction::Opcode::Add => {
                        let result_name = match &instruction.result {
                            Some(result) => match result {
                                crate::instruction::Location::Register(reg) => format!("r{}", reg),
                                crate::instruction::Location::Memory(mem) => format!("m{}", mem),
                                crate::instruction::Location::Immediate(imm) => format!("{}", imm),
                                crate::instruction::Location::Label(label) => label.to_string(),
                                crate::instruction::Location::Argument(arg) => arg.to_string(),
                            },
                            None => "".to_string(),
                        };
                        let operand1_name = match &instruction.operands[0] {
                            crate::instruction::Location::Register(reg) => format!("r{}", reg),
                            crate::instruction::Location::Memory(mem) => format!("m{}", mem),
                            crate::instruction::Location::Immediate(imm) => format!("{}", imm),
                            crate::instruction::Location::Label(label) => label.to_string(),
                            crate::instruction::Location::Argument(arg) => arg.to_string(),
                        };
                        let operand2_name = match &instruction.operands[1] {
                            crate::instruction::Location::Register(reg) => format!("r{}", reg),
                            crate::instruction::Location::Memory(mem) => format!("m{}", mem),
                            crate::instruction::Location::Immediate(imm) => format!("{}", imm),
                            crate::instruction::Location::Label(label) => label.to_string(),
                            crate::instruction::Location::Argument(arg) => arg.to_string(),
                        };
                        result += &format!(
                            "   {} {} = {} + {};\n",
                            overload.return_type, result_name, operand1_name, operand2_name
                        );
                    }
                    crate::instruction::Opcode::Return => {
                        let result_name = match &instruction.operands[0] {
                            crate::instruction::Location::Register(reg) => format!("r{}", reg),
                            crate::instruction::Location::Memory(mem) => format!("m{}", mem),
                            crate::instruction::Location::Immediate(imm) => format!("{}", imm),
                            crate::instruction::Location::Label(label) => label.to_string(),
                            crate::instruction::Location::Argument(arg) => format!("arg{}", arg),
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
