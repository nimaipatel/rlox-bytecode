use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RuntimeError {
    OperandMustBeNumber,
    OperandsMustBeNumber,
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::OperandMustBeNumber => write!(f, "Operand should be number"),
            RuntimeError::OperandsMustBeNumber => write!(f, "Operands should be number"),
        }
    }
}
