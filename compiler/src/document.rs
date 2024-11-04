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
    overload: usize,
}

pub struct Document {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
}
