#[cfg(test)]
mod tests {
    use crate::{chunk::Chunk, opcode::OpCode, vm::VM};

    #[test]
    fn test_binary_ops() {
        let mut chunk = Chunk::default();
        let mut vm = VM::new();
        chunk.write_constant(1.2.into(), 123);
        chunk.write_constant(3.4.into(), 123);
        chunk.write(OpCode::Add as u8, 123);
        chunk.write_constant(5.6.into(), 123);
        chunk.write(OpCode::Divide as u8, 123);
        chunk.write(OpCode::Negate as u8, 123);
        chunk.write(OpCode::Return as u8, 123);
        let ret = vm.run_bytecode(&chunk, false).unwrap();
        assert_eq!(ret, -0.8214285714285714f64);
    }

    #[test]
    fn test_op_divide() {
        let mut chunk = Chunk::default();
        let mut vm = VM::new();
        chunk.write_constant(1.2.into(), 123);
        chunk.write_constant((-2.4).into(), 123);
        chunk.write(OpCode::Divide as u8, 123);
        chunk.write(OpCode::Return as u8, 123);
        let ret = vm.run_bytecode(&chunk, false).unwrap();
        assert_eq!(ret, -0.5f64);
    }

    #[test]
    fn test_op_subtract() {
        let mut chunk = Chunk::default();
        let mut vm = VM::new();
        chunk.write_constant(100f64.into(), 123);
        chunk.write_constant(10f64.into(), 123);
        chunk.write(OpCode::Subtract as u8, 123);
        chunk.write(OpCode::Return as u8, 123);
        let ret = vm.run_bytecode(&chunk, false).unwrap();
        assert_eq!(ret, 90f64);
    }

    #[test]
    fn test_op_add() {
        let mut chunk = Chunk::default();
        let mut vm = VM::new();
        chunk.write_constant(100f64.into(), 123);
        chunk.write_constant(10f64.into(), 123);
        chunk.write(OpCode::Subtract as u8, 123);
        vm.run_bytecode(&chunk, false).unwrap();
        assert_eq!(vm.stack.last().unwrap(), &90f64);
    }
}
