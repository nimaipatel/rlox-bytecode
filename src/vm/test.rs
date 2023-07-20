#[cfg(test)]
mod tests {
    use crate::{chunk::Chunk, opcode::OpCode, vm::VM};

    #[test]
    fn test_binary_ops() {
        let mut vm = VM::new();
        vm.chunk.write_constant(1.2, 123);
        vm.chunk.write_constant(3.4, 123);
        vm.chunk.write(OpCode::Add as u8, 123);
        vm.chunk.write_constant(5.6, 123);
        vm.chunk.write(OpCode::Divide as u8, 123);
        vm.chunk.write(OpCode::Negate as u8, 123);
        vm.chunk.write(OpCode::Return as u8, 123);
        let ret = vm.run_bytecode(false).unwrap();
        assert_eq!(ret, -0.8214285714285714f64);
    }

    #[test]
    fn test_op_divide() {
        let mut vm = VM::new();
        vm.chunk.write_constant(1.2, 123);
        vm.chunk.write_constant(-2.4, 123);
        vm.chunk.write(OpCode::Divide as u8, 123);
        vm.chunk.write(OpCode::Return as u8, 123);
        let ret = vm.run_bytecode(false).unwrap();
        assert_eq!(ret, -0.5f64);
    }

    #[test]
    fn test_op_subtract() {
        let mut vm = VM::new();
        vm.chunk.write_constant(100f64, 123);
        vm.chunk.write_constant(10f64, 123);
        vm.chunk.write(OpCode::Subtract as u8, 123);
        vm.chunk.write(OpCode::Return as u8, 123);
        let ret = vm.run_bytecode(false).unwrap();
        assert_eq!(ret, 90f64);
    }
    
    #[test]
    fn test_op_add() {
        let mut vm = VM::new();
        vm.chunk.write_constant(100f64, 123);
        vm.chunk.write_constant(10f64, 123);
        vm.chunk.write(OpCode::Subtract as u8, 123);
        vm.run_bytecode(false).unwrap();
        assert_eq!(vm.stack.last().unwrap(), &90f64);
    }
}