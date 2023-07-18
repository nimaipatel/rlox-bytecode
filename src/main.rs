mod chunk;
mod opcode;
mod value;

use chunk::Chunk;
use value::Value;

fn main() {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(Value::Double(1.2));
    chunk.write(opcode::OpCode::Constant as u8, 1);
    chunk.write(constant, 2);
    chunk.write(opcode::OpCode::Return as u8, 3);
    chunk.write(opcode::OpCode::Return as u8, 3);
    chunk.write(opcode::OpCode::Return as u8, 3);
    chunk.write(opcode::OpCode::Return as u8, 3);
    chunk.write(opcode::OpCode::Return as u8, 3);
    chunk.write(opcode::OpCode::Return as u8, 5);
    chunk.write(opcode::OpCode::Return as u8, 5);
    chunk.write(opcode::OpCode::Return as u8, 6);
    dbg!(&chunk);
    chunk.disassemble("test chunk");
}
