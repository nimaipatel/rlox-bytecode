use crate::byte_string::Byte;
use crate::chunk::Chunk;
use crate::expr::Expr;
use crate::opcode::OpCode;
use crate::token_type::TokenType;
use crate::value::Value;

fn emit_byte(chunk: &mut Chunk, byte: Byte, line: usize) {
    chunk.write(byte, line);
}

fn emit_bytes(chunk: &mut Chunk, byte1: Byte, byte2: Byte, line: usize) {
    chunk.write(byte1, line);
    chunk.write(byte2, line);
}

fn emit_constant(chunk: &mut Chunk, value: Value, line: usize) {
    chunk.write_constant(value, line);
}

fn compile_expr<'a>(chunk: &mut Chunk, expr: &'a Expr<'a>) {
    match expr {
        Expr::NumericLiteral(n) => emit_constant(chunk, Value::Number(*n), 0), // TODO: use the actual line number
        Expr::Unary { op, expr } => {
            compile_expr(chunk, expr);
            match op.token_type {
                TokenType::Minus => emit_byte(chunk, OpCode::Negate as u8, op.line),
                TokenType::Bang => emit_byte(chunk, OpCode::Not as u8, op.line),
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
                TokenType::EqualEqual => emit_byte(chunk, OpCode::Equal as u8, op.line),
                TokenType::Greater => emit_byte(chunk, OpCode::Greater as u8, op.line),
                TokenType::Less => emit_byte(chunk, OpCode::Less as u8, op.line),
                TokenType::BangEqual => {
                    emit_bytes(chunk, OpCode::Equal as u8, OpCode::Not as u8, op.line)
                }
                TokenType::GreaterEqual => {
                    emit_bytes(chunk, OpCode::Less as u8, OpCode::Not as u8, op.line)
                }
                TokenType::LessEqual => {
                    emit_bytes(chunk, OpCode::Greater as u8, OpCode::Not as u8, op.line)
                }
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr) => compile_expr(chunk, expr),
        Expr::NilLiteral => emit_byte(chunk, OpCode::Nil as u8, 0), // TODO: use actual line number
        Expr::BoolLiteral(true) => emit_byte(chunk, OpCode::True as u8, 0), // TODO: use actual line number
        Expr::BoolLiteral(false) => emit_byte(chunk, OpCode::False as u8, 0), // TODO: use actual line number
        _ => todo!(),
    }
}

pub fn compile<'a>(expr: &Expr<'a>, chunk: &mut Chunk) -> Result<(), ()> {
    compile_expr(chunk, &expr);
    emit_byte(chunk, OpCode::Return as u8, 0);
    Ok(())
}
