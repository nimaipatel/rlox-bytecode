use std::fmt::{Debug, Display};
use std::str;

use crate::token_type::TokenType;

type ByteString = [u8];

pub struct Token<'a> {
    token_type: TokenType,
    pub lexeme: &'a ByteString,
    line: usize,
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lexeme_byte_string = match String::from_utf8(self.lexeme.into()) {
            Ok(string) => string,
            Err(_) => format!("<Couldn't decode bytes {:?} as utf8>", self.lexeme),
        };
        f.debug_struct("Token")
            .field("token_type", &self.token_type)
            .field("lexeme", &lexeme_byte_string)
            .field("line", &self.line)
            .finish()
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a ByteString, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}
