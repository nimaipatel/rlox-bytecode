mod chunk;
mod compiler;
mod error;
mod opcode;
mod lazy_scanner;
mod scanner;
mod token;
mod token_type;
mod value;
mod vm;
mod byte_string;

use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, Read, Write},
};

use chunk::Chunk;
use opcode::OpCode;
use value::Value;
use vm::VM;

use crate::compiler::compile;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    match &args[..] {
        [_] => run_prompt()?,
        [_, script_name] => run_file(&script_name)?,
        [prog_name, ..] => println!("Usage: {} [script]", prog_name),
        [] => unreachable!(),
    }
    Ok(())
}

fn run_file(script_name: &str) -> io::Result<()> {
    let vm = VM::new();
    let mut file = File::open(script_name)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    compile(&source);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    // we need to incrementally update the chunk to make this work
    let mut input_history: Vec<String> = Vec::new();
    let vm = VM::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        if line.is_empty() {
            break;
        } else {
            compile(&line);
            input_history.push(line);
        }
    }
    Ok(())
}
