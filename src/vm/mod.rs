mod test;

use std::borrow::BorrowMut;

use crate::{chunk::Chunk, error::InterpretError, opcode::OpCode, value::Value};

type Stack = Vec<Value>;
type IP = usize;

static STACK_UNDERFLOW: &'static str = "Tried poping from empty stack";

#[derive(Debug)]
pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: IP,
    stack: Stack,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn read_byte(chunk: &Chunk, ip: &mut IP) -> u8 {
        let instruction = chunk.code[*ip];
        *ip += 1;
        instruction
    }

    // fn read_byte(&mut self) -> u8 {
    //     let instruction = self.chunk.code[self.ip];
    //     self.ip += 1;
    //     instruction
    // }

    fn read_constant(chunk: &'a Chunk, ip: &mut IP) -> &'a Value {
        &chunk.constants[Self::read_byte(chunk, ip) as usize]
    }

    fn read_constant_long(chunk: &'a Chunk, ip: &mut IP) -> &'a Value {
        let h = Self::read_byte(chunk, ip);
        let m = Self::read_byte(chunk, ip);
        let l = Self::read_byte(chunk, ip);
        let idx = u32::from_be_bytes([0, h, m, l]);
        &chunk.constants[idx as usize]
    }

    pub fn run(&mut self, debug: bool) -> Result<Value, InterpretError> {
        dbg!(&self.chunk.code.len());
        loop {
            if debug {
                // TODO: make this compile time
                print!("[TRACE] ");
                print!("          ");
                print!("stack: {:?}", self.stack);
                println!();
                print!("[TRACE] ");
                self.chunk.disassemble_instruction(self.ip);
            }
            let byte = Self::read_byte(self.chunk, &mut self.ip);
            match byte.into() {
                OpCode::Return => {
                    let ret = Self::pop_unsafe(&mut self.stack);
                    println!("{}", ret);
                    return Ok(ret);
                }
                OpCode::Constant => {
                    let constant = Self::read_constant(self.chunk, &mut self.ip);
                    self.stack.push(*constant);
                }
                OpCode::ConstantLong => {
                    let constant = Self::read_constant_long(self.chunk, &mut self.ip);
                }
                OpCode::Negate => {
                    let last_ref = self.stack.last_mut().expect(STACK_UNDERFLOW);
                    *last_ref = -*last_ref;
                }
                OpCode::Add => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a + b);
                }
                OpCode::Subtract => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a - b);
                }
                OpCode::Multiply => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a * b);
                }
                OpCode::Divide => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a / b);
                }
            }
        }
    }

    fn pop_unsafe(stack: &mut Stack) -> Value {
        stack.pop().expect(STACK_UNDERFLOW)
    }

    fn pop_twice_unsafe(stack: &mut Stack) -> (Value, Value) {
        let b = stack.pop().expect(STACK_UNDERFLOW);
        let a = stack.pop().expect(STACK_UNDERFLOW);
        (a, b)
    }
}
