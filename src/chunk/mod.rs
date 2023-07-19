mod test;

use std::u8;

use crate::chunk;
use crate::opcode::OpCode;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct LineInfo {
    count: usize,
    number: usize,
}

#[derive(Default, Debug)]
pub struct Chunk {
    pub lines: Vec<LineInfo>,
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
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

    pub fn write_constant(&mut self, value: Value, line_number: usize) {
        if self.constants.len() < u8::MAX as usize + 1 {
            self.write(OpCode::Constant as u8, line_number);
            self.constants.push(value);
            self.write((self.constants.len() - 1) as u8, line_number);
        } else {
            self.constants.push(value);
            self.write(OpCode::ConstantLong as u8, line_number);
            let [.., h, m, l] = (self.constants.len() - 1).to_be_bytes();
            self.write(h as u8, line_number);
            self.write(m as u8, line_number);
            self.write(l as u8, line_number);
        }
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

    pub fn disassemble_instruction(&self, offset: usize) -> Option<usize> {
        print!("{:04} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.get_line(offset));
        }
        if let Some(instruction) = self.code.get(offset) {
            let instruction: OpCode = (*instruction).into();
            match instruction {
                OpCode::Constant => {
                    let constant_index = self.code[offset + 1];
                    println!(
                        "{:16} {constant_index} {:?}",
                        instruction, self.constants[constant_index as usize]
                    );
                    Some(offset + 2)
                }
                OpCode::ConstantLong => {
                    let h = self.code[offset + 1];
                    let m = self.code[offset + 2];
                    let l = self.code[offset + 3];
                    let constant_index = u32::from_be_bytes([0, h, m, l]);
                    println!(
                        "{:16} {constant_index} {:?}",
                        instruction, self.constants[constant_index as usize]
                    );
                    Some(offset + 4)
                }
                simple_instruction => {
                    println!("{:16}", simple_instruction);
                    Some(offset + 1)
                }
            }
        } else {
            None
        }
    }
}
