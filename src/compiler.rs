use crate::byte_string::Byte;
use crate::chunk::Chunk;
use crate::expr::Expr;
use crate::opcode::OpCode;
use crate::parser::parse_expression;
use crate::scanner::scan;
use crate::token_type::TokenType;
use crate::value::Value;
use std::str;

fn emit_byte(chunk: &mut Chunk, byte: Byte, line: usize) {
    chunk.write(byte, line);
}

fn emit_constant(chunk: &mut Chunk, value: Value, line: usize) {
    chunk.write_constant(value, line);
}

fn compile_expr<'a>(chunk: &mut Chunk, expr: &'a Expr<'a>) {
    match expr {
        Expr::NumericLiteral(n) => emit_constant(chunk, *n, 0), // TODO: use the actual line number
        Expr::Unary { op, expr } => {
            compile_expr(chunk, expr);
            match op.token_type {
                TokenType::Minus => emit_byte(chunk, OpCode::Negate as u8, op.line),
                _ => unreachable!(),
            }
        }
        Expr::Binary { left, op, right } => {
            compile_expr(chunk, left);
            compile_expr(chunk, right);
            match op.token_type {
                TokenType::Plus => emit_byte(chunk, OpCode::Add as u8, op.line),
                TokenType::Minus => emit_byte(chunk, OpCode::Subtract as u8, op.line),
                TokenType::Star => emit_byte(chunk, OpCode::Multiply as u8, op.line),
                TokenType::Slash => emit_byte(chunk, OpCode::Divide as u8, op.line),
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr) => compile_expr(chunk, expr),
        _ => todo!(),
    }
}

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), ()> {
    let tokens = scan(source.as_bytes()).unwrap();
    let (expr, _) = parse_expression(&tokens[..], 0).unwrap();
    compile_expr(chunk, &expr);
    emit_byte(chunk, OpCode::Return as u8, 0);
    Ok(())
}
