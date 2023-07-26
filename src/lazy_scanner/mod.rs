mod scan_error;
use std::cell::RefCell;

use crate::{
    byte_string::{Byte, ByteSlice},
    token::Token,
    token_type::{string_to_keyword, TokenType},
};

use self::scan_error::ScanError;

pub struct Scanner<'a> {
    bytes: &'a ByteSlice,
    start: usize,
    current: RefCell<usize>,
    line: RefCell<usize>,
}

fn is_alpha(byte: Byte) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

impl<'a> Scanner<'a> {
    pub fn new(bytes: &'a ByteSlice) -> Self {
        Self {
            bytes,
            start: 0,
            current: RefCell::new(0),
            line: RefCell::new(1),
        }
    }

    fn is_at_end(&self) -> bool {
        *self.current.borrow() + 1 == self.bytes.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token::new(
            token_type,
            &self.bytes[self.start..*self.current.borrow()],
            *self.line.borrow(),
        )
    }

    fn advance(&self) -> u8 {
        let mut current = self.current.borrow_mut();
        let byte = self.bytes[*current];
        *current += 1;
        byte
    }

    fn matchh(&self, byte: Byte) -> bool {
        if self.bytes[*self.current.borrow()] != byte || self.is_at_end() {
            false
        } else {
            *self.current.borrow_mut() += 1;
            true
        }
    }

    fn make_token_cond(&self, byte: Byte, if_true: TokenType, if_false: TokenType) -> Token<'a> {
        if self.matchh(byte) {
            self.make_token(if_true)
        } else {
            self.make_token(if_false)
        }
    }

    fn peek(&self) -> Byte {
        self.bytes[*self.current.borrow()]
    }

    fn peekpeek(&self) -> Byte {
        self.bytes[*self.current.borrow() + 1]
    }

    fn skip_white_space(&self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    *self.line.borrow_mut() += 1;
                    self.advance();
                }
                b'/' if self.peekpeek() == b'/' => {
                    while !self.is_at_end() && self.peek() != b'\n' {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn make_token_string(&self) -> Result<Token<'a>, ScanError> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                *self.line.borrow_mut() += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Err(ScanError::UnterminatedString(self.start))
        } else {
            self.advance();
            Ok(self.make_token(TokenType::String))
        }
    }

    fn make_digit(&self) -> Token<'a> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peekpeek().is_ascii_digit() {
            self.advance(); // consume the `.`
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn make_ident(&self) -> Token<'a> {
        while is_alpha(self.peek()) || self.peek().is_ascii_digit() {
            self.advance();
        }
        let lexeme = &self.bytes[self.start..*self.current.borrow()];
        match string_to_keyword(lexeme) {
            Some(token_type) => self.make_token(token_type),
            None => self.make_token(TokenType::Identifier),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_white_space();
        self.start = *self.current.borrow();
        if self.is_at_end() {
            return None;
        }
        let token = match self.advance() {
            b'(' => Ok(self.make_token(TokenType::LeftParen)),
            b')' => Ok(self.make_token(TokenType::RightParen)),
            b'{' => Ok(self.make_token(TokenType::LeftBrace)),
            b'}' => Ok(self.make_token(TokenType::RightBrace)),
            b';' => Ok(self.make_token(TokenType::Semicolon)),
            b',' => Ok(self.make_token(TokenType::Comma)),
            b'.' => Ok(self.make_token(TokenType::Dot)),
            b'-' => Ok(self.make_token(TokenType::Minus)),
            b'+' => Ok(self.make_token(TokenType::Plus)),
            b'/' => Ok(self.make_token(TokenType::Slash)),
            b'*' => Ok(self.make_token(TokenType::Star)),
            b'!' => Ok(self.make_token_cond(b'=', TokenType::BangEqual, TokenType::Bang)),
            b'=' => Ok(self.make_token_cond(b'=', TokenType::EqualEqual, TokenType::Equal)),
            b'<' => Ok(self.make_token_cond(b'=', TokenType::LessEqual, TokenType::Less)),
            b'>' => Ok(self.make_token_cond(b'=', TokenType::GreaterEqual, TokenType::Greater)),
            b'"' => self.make_token_string(),
            digit if digit.is_ascii_digit() => Ok(self.make_digit()),
            alpha if is_alpha(alpha) => Ok(self.make_ident()),
            unknown_byte => Err(ScanError::UnknownByte(unknown_byte)),
        };
        Some(token)
    }
}
