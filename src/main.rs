mod chunk;
mod error;
mod opcode;
mod value;
mod vm;

use chunk::Chunk;
use opcode::OpCode;
use value::Value;
use vm::VM;

fn main() {
    let mut chunk = Chunk::default();
    chunk.write_constant(1.2, 123);
    chunk.write_constant(3.4, 123);
    chunk.write(OpCode::Add as u8, 123);
    chunk.write_constant(5.6, 123);
    chunk.write(OpCode::Divide as u8, 123);
    chunk.write(OpCode::Negate as u8, 123);
    chunk.write(OpCode::Return as u8, 123);
    let mut vm = VM::new(&chunk);
    vm.run(true);
}
