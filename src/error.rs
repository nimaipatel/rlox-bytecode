use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RuntimeError {
    OperandMustBeNumber,
    OperandsMustBeNumber,
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
