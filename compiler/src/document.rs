use std::rc::Rc;

use crate::{
    code_generator::CodeGenerator,
    function::Function,
    instruction::{Instruction, Location, Opcode},
};

pub struct Connection {
    from_id: usize,
    to_id: usize,
    to_input: usize,
}

pub struct Node {
    name: String,
    function: Rc<Function>,
    generator: Rc<dyn CodeGenerator>,
    overload: usize,
}

impl Node {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn overload(&self) -> usize {
        self.overload
    }

    pub fn generator(&self) -> &dyn CodeGenerator {
        &*self.generator
    }
}

pub struct Document {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(
        &mut self,
        name: &str,
        function: Rc<Function>,
        generator: Rc<dyn CodeGenerator>,
        overload: usize,
    ) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            name: name.to_string(),
            generator,
            function,
            overload,
        });
        id
    }

    pub fn add_connection(&mut self, from_id: usize, to_id: usize, to_input: usize) {
        self.connections.push(Connection {
            from_id,
            to_id,
            to_input,
        });
    }

    pub fn for_each_node<F>(&self, mut f: F)
    where
        F: FnMut(usize, &Node),
    {
        for (id, node) in self.nodes.iter().enumerate() {
            f(id, node);
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
