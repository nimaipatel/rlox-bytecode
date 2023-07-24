use std::fmt::Debug;

use crate::byte_string::ByteString;
use crate::token_type::TokenType;

#[derive(Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a ByteString,
    pub line: usize,
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

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a ByteString, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}
