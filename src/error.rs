use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum InterpretError {}

impl Error for InterpretError {}

impl Display for InterpretError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
