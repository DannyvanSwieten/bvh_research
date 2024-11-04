use crate::{
    function::FunctionOverload,
    instruction::{Instruction, Location},
};

pub struct GeneratorContext {
    next_free_register: u32,
    pub input_locations: Vec<Location>,
}

impl GeneratorContext {
    pub fn new() -> Self {
        Self {
            next_free_register: 0,
            input_locations: Vec::new(),
        }
    }

    pub fn location(&self, arg: usize) -> &Location {
        &self.input_locations[arg]
    }

    pub fn next_free_register(&mut self) -> Location {
        let result = Location::Register(self.next_free_register);
        self.next_free_register += 1;
        result
    }
}

impl Default for GeneratorContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CodeGenerator {
    fn output(
        &self,
        index: usize,
        overload: &FunctionOverload,
        ctx: &mut GeneratorContext,
    ) -> Vec<Instruction>;
}
