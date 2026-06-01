use crate::vm::VM;
use crate::vm::vm::InterpretResult;
use chunk::*;
use std::fs;
use std::io::Write;
use std::process::exit;

mod chunk;
pub mod compiler;
pub mod vm;

fn main() {
    // let mut chunk = chunk::Chunk::new();
    // let len = 2;
    // for i in 0..len {
    //     chunk.write_constant(1.2+i as f64,i+1);
    // }
    // chunk.write_chunk(u8::from(OpCode::OpAdd),3);
    // for i in len..len+2{
    //     chunk.write_constant(2.4+i as f64,i+1);
    //     chunk.write_chunk(u8::from(OpCode::OpMul),i+1);
    // }
    // chunk.write_chunk(u8::from(OpCode::OpNegate),9);
    // chunk.write_constant(10.0,10);
    // chunk.write_chunk(u8::from(OpCode::OpDiv),11);
    // chunk.write_chunk(u8::from(OpCode::OpReturn), 100);
    let mut vm = VM::new();
    // vm.interpret(chunk);
    let args = std::env::args();
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args.last().unwrap()),
        _ => {
            eprintln!("Usage: rLox [path]\n");
            exit(64);
        }
    }
}
fn repl(vm: &mut VM) {
    let mut line = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        line.clear();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                vm.interpret(&line);
            }
            Err(err) => {
                eprintln!("repl:{}", err);
            }
        }
    }
}
fn run_file(vm: &mut VM, path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("run_file:{}", err);
            exit(74);
        }
    };
    match vm.interpret(&path) {
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
        _ => {}
    }
}
