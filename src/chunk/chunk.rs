use super::line::*;
use super::value::*;
use crate::chunk::OpCode;

#[derive(Debug,Clone)]
pub struct Chunk {
    //uint8* code
    pub code: Vec<u8>,
    capacity: u32,
    value_array: ValueArray,
    lines: Lines,
}
fn simple_instruction(name: &str, offset: u32) -> u32 {
    println!("{}", name);
    offset + 1
}
impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            capacity: 0,
            value_array: ValueArray::new(),
            lines: Lines::new(),
        }
    }
    pub fn write_chunk(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.add_line(line);
        self.capacity = self.code.capacity() as u32;
    }
    pub fn write_constant(&mut self, value: Value, line: u32) {
        const ONE_MAX: usize = u8::MAX as usize;
        if self.value_array.count() >= ONE_MAX {
            self.code.push(OpCode::OpConstantLong as u8);
            //code low medium high 存储
            self.value_array.write_value(value);
            let idx = self.value_array.count() - 1;
            let low: u8 = (idx & 0xff) as u8;
            let medium: u8 = ((idx & 0xff00) >> 8) as u8;
            let high: u8 = ((idx & 0xff0000) >> 16) as u8;
            self.write_chunk(low, line);
            self.write_chunk(medium, line);
            self.write_chunk(high, line);
        } else {
            self.code.push(OpCode::OpConstant as u8);
            self.value_array.write_value(value);
            self.write_chunk(self.value_array.count() as u8 - 1, line);
        }
        self.lines.add_line(line);
        self.capacity = self.code.capacity() as u32;
    }

    pub fn value_array(&self) -> &ValueArray {
        &self.value_array
    }
}
/**
below are all dbg fn
*/
impl Chunk {
    pub fn disassemble(&self, msg: &str) {
        println!("== {} ==", msg);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset as u32) as usize;
        }
    }
    fn constant_instruction(&self, name: &str, offset: u32) -> u32 {
        //value索引
        let const_ptr = self.code[(offset + 1) as usize];
        print!("{:<16} {:4}", name, const_ptr);
        print!(" '{}' ", self.value_array.values()[const_ptr as usize]);
        println!();
        offset + 2
    }
    fn constant_long_instruction(&self, name: &str, offset: u32) -> u32 {
        let low = self.code[(offset + 1) as usize];
        let medium = self.code[(offset + 2) as usize];
        let high = self.code[(offset + 3) as usize];
        let idx: u32 = ((high as u32) << 16) + ((medium as u32) << 8) + low as u32;
        print!("{:<16} {:4}", name, idx);
        print!(" '{}' ", self.value_array.values()[idx as usize]);
        println!();
        offset + 4
    }
    pub fn disassemble_instruction(&self, offset: u32) -> u32 {
        print!("{:04} ", offset);
        print!("{:04} ", self.lines.get_line(offset).unwrap());

        let instruction = self.code[offset as usize];
        match OpCode::try_from(instruction).unwrap() {
            OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
            OpCode::OpConstant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::OpConstantLong => self.constant_long_instruction("OP_CONSTANT_LONG", offset),
            OpCode::OpNegate => simple_instruction("OP_NEGATE", offset),
            OpCode::OpAdd => simple_instruction("OP_ADD", offset),
            OpCode::OpSub => simple_instruction("OP_SUBTRACT", offset),
            OpCode::OpMul => simple_instruction("OP_MULTI", offset),
            OpCode::OpDiv => simple_instruction("OP_DIV", offset),
            _ => {
                println!("Unknown opcode {:?}", instruction);
                offset + 1
            }
        }
    }
}
