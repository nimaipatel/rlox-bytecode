use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl Error for InterpretError {}

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
