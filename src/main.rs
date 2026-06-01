use chunk::*;
use crate::chunk::value::Value;
use crate::vm::VM;

mod chunk;
pub mod vm;

fn main() {
    let mut chunk = chunk::Chunk::new();
    let len = 2;
    for i in 0..len {
        chunk.write_constant(1.2+i as f64,i+1);
    }
    chunk.write_chunk(u8::from(OpCode::OpAdd),3);
    for i in len..len+2{
        chunk.write_constant(2.4+i as f64,i+1);
        chunk.write_chunk(u8::from(OpCode::OpMul),i+1);
    }
    chunk.write_chunk(u8::from(OpCode::OpNegate),9);
    chunk.write_constant(10.0,10);
    chunk.write_chunk(u8::from(OpCode::OpDiv),11);
    chunk.write_chunk(u8::from(OpCode::OpReturn), 100);
    let mut vm = VM::new();
    vm.interpret(chunk);
}
