use std::fmt::{Debug, Display};

use intersect::types::DataType;

#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub data_type: DataType,
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} datatype: {:?}", self.name, self.data_type)
    }
}

impl Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
#[derive(Clone)]
pub struct FunctionOverload {
    pub arguments: Vec<Argument>,
    pub return_type: DataType,
}

impl FunctionOverload {
    pub fn new() -> Self {
        Self {
            arguments: Vec::new(),
            return_type: DataType::Void,
        }
    }

    pub fn add_argument(&mut self, name: &str, data_type: DataType) {
        self.arguments.push(Argument {
            name: name.to_string(),
            data_type,
        });
    }

    pub fn with_argument(mut self, name: &str, data_type: DataType) -> Self {
        self.arguments.push(Argument {
            name: name.to_string(),
            data_type,
        });
        self
    }

    pub fn with_return_type(mut self, return_type: DataType) -> Self {
        self.return_type = return_type;
        self
    }
}

impl Default for FunctionOverload {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for FunctionOverload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, argument) in self.arguments.iter().enumerate() {
            write!(f, "\n  Argument {}: {:?}", i, argument).expect("Failed to write argument");
        }
        write!(f, "\n  Return type: {:?}", self.return_type).expect("Failed to write return type");
        Ok(())
    }
}

impl Debug for FunctionOverload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Function {
    pub name: String,
    pub overloads: Vec<FunctionOverload>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            overloads: Vec::new(),
        }
    }

    pub fn with_overload(mut self, overload: FunctionOverload) -> Self {
        self.overloads.push(overload);
        self
    }

    pub fn with_overloads(mut self, overloads: Vec<FunctionOverload>) -> Self {
        self.overloads = overloads;
        self
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function: {}", self.name).expect("Failed to write function name");
        for (i, overload) in self.overloads.iter().enumerate() {
            write!(f, "\n  Overload {}: {:?}", i, overload).expect("Failed to write overload");
        }
        Ok(())
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
