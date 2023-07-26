use std::fmt::{Display, Debug};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_rep = match self {
            OpCode::Return => "OP_RETURN",
            OpCode::Constant => "OP_CONSTANT",
            OpCode::ConstantLong => "OP_CONSTANT_LONG",
            OpCode::Negate => "OP_NEGATE",
            OpCode::Add => "OP_ADD",
            OpCode::Subtract => "OP_SUBTRACT",
            OpCode::Multiply => "OP_MULTIPLY",
            OpCode::Divide => "OP_DIVIDE",
            OpCode::Nil => "OP_NIL",
            OpCode::True => "OP_TRUE",
            OpCode::False => "OP_FALSE",
            OpCode::Not => "OP_NOT",
            OpCode::Equal => "OP_EQUAL",
            OpCode::Greater => "OP_GREATER",
            OpCode::Less => "OP_LESS",
        };
        write!(f, "{:16}", string_rep)
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
