mod test;

use crate::chunk;
use crate::opcode::OpCode;
use crate::value::Value;

#[derive(Debug, PartialEq)]
struct LineInfo {
    count: usize,
    number: usize,
}

#[derive(Default, Debug)]
pub struct Chunk {
    lines: Vec<LineInfo>,
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn write(&mut self, byte: u8, line_number: usize) {
        self.code.push(byte);
        if let Some(last_line) = self.lines.last_mut() {
            if last_line.number == line_number {
                last_line.count += 1;
                return;
            }
        }

        self.lines.push(LineInfo {
            count: 1,
            number: line_number,
        })
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1)
            .try_into()
            .expect("Number of constants greater than can be stored in `u8`")
    }

    fn get_line(&self, offset: usize) -> usize {
        let mut cumulative_position = 0;
        for line_chunk in self.lines.iter() {
            cumulative_position += line_chunk.count;
            if cumulative_position > offset {
                return line_chunk.number;
            }
        }
        0
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset).unwrap();
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> Option<usize> {
        print!("{:04} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.get_line(offset));
        }
        if let Some(instruction) = self.code.get(offset) {
            let instruction: OpCode = (*instruction).into();
            match instruction {
                OpCode::Return => {
                    println!("{:16}", "OP_RETURN");
                    Some(offset + 1)
                }
                OpCode::Constant => {
                    let constant_index = self.code[offset + 1];
                    println!(
                        "{:16} {constant_index} {:?}",
                        "OP_CONSTANT", self.constants[constant_index as usize]
                    );
                    Some(offset + 2)
                }
            }
        } else {
            None
        }
    }
}
