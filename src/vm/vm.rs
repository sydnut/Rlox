use crate::bytecode::OpCode::*;
use crate::chunk::value::Value;
use crate::chunk::*;
pub struct VM {
    chunk: Box<Chunk>,
    /// ip:指令指针,这里最好可以实现成idx
    ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
}
const STACK_MAX: usize = 1024;
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}
impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Box::new(Chunk::new()),
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }
    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        self.chunk = Box::new(chunk);
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            print!("\t\t");
            for x in &self.stack {
                print!("[");
                print!("'{}'", x);
                print!("]");
            }
            println!();
            self.chunk.disassemble_instruction(self.ip as u32);
            match OpCode::try_from(self.read_byte()).unwrap_or(OpReturn) {
                OpReturn => {
                    return InterpretResult::InterpretOk;
                }
                OpConstant => {
                    let constant = self.read_constant();
                    self.push(constant);
                    print!(" '{}' \n", constant);
                }
                OpConstantLong => {
                    let constant = self.read_constant_long();
                    self.push(constant);
                    print!(" '{}' \n", constant);
                }
                OpNegate => {
                    let top = self.top();
                    *top = -*top;
                }
                OpAdd => self.binary_op(|a, b| a + b),
                OpSub => self.binary_op(|a, b| a - b),
                OpMul => self.binary_op(|a, b| a * b),
                OpDiv => self.binary_op(|a, b| a / b),
                _ => {}
            }
        }
    }
}
// VM内部栈操作函数
impl VM {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1
    }
    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        match self.stack.pop() {
            None => {
                panic!("Stack underflow");
            }
            Some(ret) => ret,
        }
    }
    fn top(&mut self) -> &mut Value {
        &mut self.stack[self.stack_top - 1]
    }
    fn binary_op(&mut self, op: fn(f64, f64) -> f64) {
        let b = self.pop();
        let a = self.top();
        *a = op(*a, b);
    }
}
// VM内部读取字节码函数
impl VM {
    /// 返回ip指针对应字节码并移动ip指针
    fn read_byte(&mut self) -> u8 {
        let res = self.chunk.code[self.ip];
        self.ip += 1;
        res
    }
    fn read_constant(&mut self) -> f64 {
        let idx = self.read_byte() as usize;
        self.chunk.value_array().values()[idx]
    }
    fn read_constant_long(&mut self) -> f64 {
        let low = self.read_byte();
        let medium = self.read_byte();
        let high = self.read_byte();
        let idx = ((high as usize) << 16) + ((medium as usize) << 8) + low as usize;
        self.chunk.value_array().values()[idx]
    }
}
