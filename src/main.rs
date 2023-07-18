mod chunk;
mod opcode;
mod value;

use chunk::Chunk;
use value::Value;

fn main() {
    let mut chunk = Chunk::default();
    for i in 0..((u8::MAX as usize) * (u8::MAX as usize) * (u8::MAX as usize)) {
        chunk.write_constant(Value::Double(0f64), i);
    }
}
