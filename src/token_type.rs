use crate::byte_string::ByteSlice;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub fn string_to_keyword(string: &ByteSlice) -> Option<TokenType> {
    match string {
        b"and" => Some(TokenType::And),
        b"class" => Some(TokenType::Class),
        b"else" => Some(TokenType::Else),
        b"false" => Some(TokenType::False),
        b"for" => Some(TokenType::For),
        b"fun" => Some(TokenType::Fun),
        b"if" => Some(TokenType::If),
        b"nil" => Some(TokenType::Nil),
        b"or" => Some(TokenType::Or),
        b"print" => Some(TokenType::Print),
        b"return" => Some(TokenType::Return),
        b"super" => Some(TokenType::Super),
        b"this" => Some(TokenType::This),
        b"true" => Some(TokenType::True),
        b"var" => Some(TokenType::Var),
        b"while" => Some(TokenType::While),
        _ => None,
    }
}
