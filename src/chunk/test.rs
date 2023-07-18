#[cfg(test)]
mod tests {
    use crate::{
        chunk::{Chunk, LineInfo},
        opcode::OpCode,
        value::Value,
    };

    #[test]
    fn write_op_constant() {
        let mut chunk = Chunk::default();
        let constant = chunk.add_constant(Value::Double(1.2));
        chunk.write(OpCode::Constant as u8, 1);
        chunk.write(constant, 1);
        assert_eq!(chunk.code, vec![OpCode::Constant as u8, 0]);
    }

    #[test]
    fn write_op_return() {
        let mut chunk = Chunk::default();
        chunk.write(OpCode::Return as u8, 1);
        assert_eq!(chunk.code, vec![OpCode::Return as u8]);
    }

    #[test]
    fn test_line_info() {
        let mut chunk = Chunk::default();
        let constant = chunk.add_constant(Value::Double(1.2));
        chunk.write(OpCode::Constant as u8, 1);
        chunk.write(constant, 2);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 6);
        let constant = chunk.add_constant(Value::Double(2.1));
        chunk.write(OpCode::Constant as u8, 6);
        chunk.write(constant, 7);
        chunk.write(OpCode::Return as u8, 7);

        assert_eq!(
            chunk.lines,
            vec![
                LineInfo {
                    count: 1,
                    number: 1,
                },
                LineInfo {
                    count: 1,
                    number: 2,
                },
                LineInfo {
                    count: 5,
                    number: 3,
                },
                LineInfo {
                    count: 2,
                    number: 5,
                },
                LineInfo {
                    count: 2,
                    number: 6,
                },
                LineInfo {
                    count: 2,
                    number: 7,
                },
            ]
        );
    }

    #[test]
    fn test_get_line() {
        let mut chunk = Chunk::default();
        let constant = chunk.add_constant(Value::Double(1.2));
        chunk.write(OpCode::Constant as u8, 1);
        chunk.write(constant, 2);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 6);
        let constant = chunk.add_constant(Value::Double(2.1));
        chunk.write(OpCode::Constant as u8, 6);
        chunk.write(constant, 7);
        chunk.write(OpCode::Return as u8, 7);

        assert_eq!(chunk.get_line(0), 1);
        assert_eq!(chunk.get_line(1), 2);
        assert_eq!(chunk.get_line(6), 3);
        assert_eq!(chunk.get_line(7), 5);
        assert_eq!(chunk.get_line(12), 7);
    }
}


