mod test;

use std::cell::Cell;

use crate::{
    chunk::Chunk, compiler::compile, error::RuntimeError, opcode::OpCode, parser::parse_expression,
    scanner::scan, value::Value,
};

type Stack = Vec<Value>;
type IP = usize;

static STACK_UNDERFLOW: &'_ str = "Tried poping from empty stack";

#[derive(Debug)]
pub struct VM {
    // pub chunk: RefCell<Chunk>,
    ip: Cell<IP>,
    stack: Stack,
}

impl VM {
    pub fn new() -> Self {
        Self {
            // chunk: RefCell::new(Chunk::default()),
            ip: Cell::new(0),
            stack: Vec::with_capacity(256),
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn read_byte(&self, chunk: &Chunk) -> u8 {
        let ip = self.ip.get();
        let instruction = chunk.code[ip];
        self.ip.set(ip + 1);
        instruction
    }

    fn read_constant(&self, chunk: &Chunk) -> Value {
        chunk.constants[self.read_byte(chunk) as usize]
    }

    fn read_constant_long(&self, chunk: &Chunk) -> Value {
        let h = self.read_byte(chunk);
        let m = self.read_byte(chunk);
        let l = self.read_byte(chunk);
        let idx = u32::from_be_bytes([0, h, m, l]);
        chunk.constants[idx as usize]
    }

    pub fn run_bytecode(&mut self, chunk: &Chunk, debug: bool) -> Result<Value, RuntimeError> {
        while self.ip.get() < chunk.code.len() {
            if debug {
                // TODO: make this compile time
                print!("[TRACE] ");
                print!("          ");
                print!("stack: {:?}", self.stack);
                println!();
                print!("[TRACE] ");
                chunk.disassemble_instruction(self.ip.get());
            }
            let byte = self.read_byte(chunk);
            match byte.into() {
                OpCode::Return => {
                    let ret = Self::pop_unsafe(&mut self.stack);
                    println!("{}", ret);
                    return Ok(ret);
                }
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                OpCode::ConstantLong => {
                    let constant = self.read_constant_long(chunk);
                    self.stack.push(constant);
                }
                OpCode::Negate => {
                    let last_ref = self.stack.last_mut().expect(STACK_UNDERFLOW);
                    *last_ref = last_ref.negate()?;
                }
                OpCode::Add => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a.add(b)?);
                }
                OpCode::Subtract => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a.subtract(b)?);
                }
                OpCode::Multiply => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a.multiply(b)?);
                }
                OpCode::Divide => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push(a.divide(b)?);
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::Not => {
                    let last_ref = self.stack.last_mut().expect(STACK_UNDERFLOW);
                    let as_bool: bool = (*last_ref).into();
                    *last_ref = Value::Boolean(!as_bool);
                }
                OpCode::Equal => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    self.stack.push((a == b).into())
                }
                OpCode::Greater => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    match (a, b) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            self.stack.push(Value::Boolean(n1 > n2))
                        }
                        _ => return Err(RuntimeError::OperandsMustBeNumber),
                    }
                }
                OpCode::Less => {
                    let (a, b) = Self::pop_twice_unsafe(&mut self.stack);
                    match (a, b) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            self.stack.push(Value::Boolean(n1 < n2))
                        }
                        _ => return Err(RuntimeError::OperandsMustBeNumber),
                    }
                }
            }
        }
        Ok(0f64.into())
    }

    fn pop_unsafe(stack: &mut Stack) -> Value {
        stack.pop().expect(STACK_UNDERFLOW)
    }

    fn pop_twice_unsafe(stack: &mut Stack) -> (Value, Value) {
        let b = stack.pop().expect(STACK_UNDERFLOW);
        let a = stack.pop().expect(STACK_UNDERFLOW);
        (a, b)
    }

    pub fn run(
        &mut self,
        source: &str,
        chunk: &mut Chunk,
        debug: bool,
    ) -> Result<Value, RuntimeError> {
        let tokens = scan(source.as_bytes()).unwrap();
        let (expr, _) = parse_expression(&tokens, 0).unwrap();
        compile(&expr, chunk).unwrap();
        self.run_bytecode(chunk, debug)
    }
}
