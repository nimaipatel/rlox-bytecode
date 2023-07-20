use std::str;
use crate::scanner::Scanner;
use crate::token::Token;

pub fn compile(source: &str) {
    dbg!(&source);
    let scanner = Scanner::new(source.as_bytes());
    let tokens = scanner.collect::<Vec<_>>();
    dbg!(&tokens);
}