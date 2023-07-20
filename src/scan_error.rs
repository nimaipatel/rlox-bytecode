use std::{error::Error, fmt::Display};

type Byte = u8;

#[derive(Debug)]
pub enum ScanError {
    UnknownByte(Byte),
    UnterminatedString(usize),
}

impl Error for ScanError {}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}