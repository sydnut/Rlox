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
    let mut vm = VM::new();
    // vm.interpret(chunk);
    let args = std::env::args();
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args.last().unwrap()),
        _ => {
            println!("wrong args:{:?}",args);
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
                line.push(char::from(b'\0'));
                vm.interpret(&line);
            }
            Err(err) => {
                eprintln!("repl:{}", err);
            }
        }
    }
}
fn run_file(vm: &mut VM, path: &str) {
    let mut source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("run_file:{}", err);
            exit(74);
        }
    };
    source.push('\0');
    match vm.interpret(&source) {
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
        _ => {}
    }
}
