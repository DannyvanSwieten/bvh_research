use std::fmt::Debug;

pub enum Opcode {
    Call,
    Load,
    Add,
    Return,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Opcode::Call => write!(f, "Call"),
            Opcode::Load => write!(f, "Load"),
            Opcode::Add => write!(f, "Add"),
            Opcode::Return => write!(f, "Return"),
        }
    }
}

#[derive(Clone)]
pub enum Location {
    Register(u32),
    Memory(u32),
    Immediate(i32),
    Label(String),
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Location::Register(reg) => write!(f, "Register({})", reg),
            Location::Memory(mem) => write!(f, "Memory({})", mem),
            Location::Immediate(imm) => write!(f, "Immediate({})", imm),
            Location::Label(label) => write!(f, "Label({})", label),
        }
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Location>,
    pub result: Option<Location>,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Instruction {{ opcode: {:?}, operands: {:?}, result: {:?} }}",
            self.opcode, self.operands, self.result
        )
    }
}
