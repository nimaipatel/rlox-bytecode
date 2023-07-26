use crate::{byte_string::ByteSlice, token::Token};

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    StringLiteral(&'a ByteSlice),
    NumericLiteral(f64), // TODO: make this store bytestring only
    BoolLiteral(bool),   // TODO: make two seperate literals for true and false
    NilLiteral,
    Logical {
        left: Box<Expr<'a>>,
        op: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Unary {
        op: &'a Token<'a>,
        expr: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        op: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping(Box<Expr<'a>>),
    Variable(&'a Token<'a>),
    Assign {
        name: &'a Token<'a>,
        value: Box<Expr<'a>>,
    },
    Call {
        callee: Box<Expr<'a>>,
        // we also store closing parentheses of function calls to
        // report runtime error caused by function calls
        paren: &'a Token<'a>,
        arguments: Vec<Expr<'a>>,
    },
}
