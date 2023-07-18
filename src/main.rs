mod chunk;
mod opcode;
mod value;

use chunk::Chunk;
use value::Value;

fn main() {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(Value::Double(1.2));
    chunk.write(opcode::OpCode::Constant as u8, 1);
    chunk.write(constant, 1);
    chunk.write(opcode::OpCode::Return as u8, 1);
    chunk.disassemble("test chunk");
}
