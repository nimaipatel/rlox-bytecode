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
        chunk.write_constant(1.2, 1);
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
        chunk.write_constant(1.2, 1);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 6);
        chunk.write_constant(1.2, 6);
        chunk.write(OpCode::Return as u8, 7);

        assert_eq!(
            chunk.lines,
            vec![
                LineInfo {
                    count: 2,
                    number: 1,
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
                    count: 3,
                    number: 6,
                },
                LineInfo {
                    count: 1,
                    number: 7,
                },
            ]
        );
    }

    #[test]
    fn test_get_line() {
        let mut chunk = Chunk::default();
        chunk.write_constant(1.2, 1);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 3);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 5);
        chunk.write(OpCode::Return as u8, 6);
        chunk.write_constant(1.2, 6);
        chunk.write(OpCode::Return as u8, 7);

        assert_eq!(chunk.get_line(0), 1);
        assert_eq!(chunk.get_line(1), 1);
        assert_eq!(chunk.get_line(6), 3);
        assert_eq!(chunk.get_line(7), 5);
        assert_eq!(chunk.get_line(12), 7);
    }

    #[test]
    fn many_constants() {
        let mut chunk = Chunk::default();
        for i in 0..(255 * 255) {
            chunk.write_constant(0f64, i);
        }

        // last index that can be stored in 1 byte
        assert_eq!(chunk.code[255 * 2 + 0], OpCode::Constant as u8);
        assert_eq!(chunk.code[255 * 2 + 1], 255);

        // first index which needs 3 bytes for storing
        assert_eq!(chunk.code[256 * 2 + 0], OpCode::ConstantLong as u8);
        let h = chunk.code[256 * 2 + 1];
        let m = chunk.code[256 * 2 + 2];
        let l = chunk.code[256 * 2 + 3];
        assert_eq!(256, u32::from_be_bytes([0, h, m, l]));

        // last index in constant vector
        let last_idx = chunk.code.len() - 1;
        assert_eq!(chunk.code[last_idx - 3], OpCode::ConstantLong as u8);
        let h = chunk.code[last_idx - 2];
        let m = chunk.code[last_idx - 1];
        let l = chunk.code[last_idx - 0];
        assert_eq!(255 * 255 - 1, u32::from_be_bytes([0, h, m, l]));
    }
}
