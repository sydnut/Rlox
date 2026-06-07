use crate::bytecode::OpCode::*;
use crate::chunk::obj::Object;
use crate::chunk::value::Value;
use crate::chunk::*;
use crate::compiler::compiler::compile;
use std::rc::Rc;

pub struct VM {
    chunk: Chunk,
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
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }
    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        match compile(source) {
            Some(chunk) => {
                self.chunk = chunk;
            }
            None => return InterpretResult::InterpretCompileError,
        }
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
                    print!(" '{}' \n", constant);
                    self.push(constant);
                }
                OpConstantLong => {
                    let constant = self.read_constant_long();
                    print!(" '{}' \n", constant);
                    self.push(constant);
                }
                OpNegate => {
                    let top = self.top();
                    if let Value::Double(value) = *top {
                        *top = Value::Double(-value);
                    } else {
                        self.runtime_error("Operand must be a number.");
                    }
                }
                OpAdd => match self.add_values() {
                    None => {}
                    Some(err) => return err,
                },
                OpSub => match self.binary_number_op(|a, b| a - b) {
                    None => {}
                    Some(err) => return err,
                },
                OpMul => match self.binary_number_op(|a, b| a * b) {
                    None => {}
                    Some(err) => return err,
                },
                OpDiv => match self.binary_number_op(|a, b| a / b) {
                    None => {}
                    Some(err) => return err,
                },
                OpNil => self.push(Value::Nil),
                OpTrue => {
                    self.push(Value::Boolean(true));
                }
                OpFalse => {
                    self.push(Value::Boolean(false));
                }
                OpNot => {
                    let top = self.top();
                    *top = Value::Boolean(!top.is_truthy());
                }
                OpEqual => {
                    let b = self.pop();
                    let a = self.top();
                    *a = Value::Boolean(*a == b);
                }
                OpGreater => {
                    let b = self.pop();
                    let a = self.top();
                    *a = Value::Boolean(*a > b);
                }
                OpLess => {
                    let b = self.pop();
                    let a = self.top();
                    *a = Value::Boolean(*a < b);
                }
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
    fn add_values(&mut self) -> Option<InterpretResult> {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (Value::Double(a), Value::Double(b)) => {
                self.push(Value::Double(a + b));
                None
            }
            (Value::Obj(a), Value::Obj(b)) => match (a.as_ref(), b.as_ref()) {
                (Object::String(a), Object::String(b)) => {
                    let mut result = a.clone();
                    result.push_str(b);
                    self.push(Value::Obj(Rc::new(Object::String(result))));
                    None
                }
            },
            _ => {
                self.runtime_error("Operands must be two numbers or two strings.");
                Some(InterpretResult::InterpretRuntimeError)
            }
        }
    }
    fn binary_number_op(&mut self, op: fn(f64, f64) -> f64) -> Option<InterpretResult> {
        let b = self.pop();
        let mut a = self.top();
        if let (Value::Double(va), Value::Double(vb)) = (&mut a, b) {
            *a = Value::Double(op(*va, vb));
            None
        } else {
            self.runtime_error("Operands must be a number.");
            Some(InterpretResult::InterpretRuntimeError)
        }
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
    fn read_constant(&mut self) -> Value {
        let idx = self.read_byte() as usize;
        self.chunk.value_array().values()[idx].clone()
    }
    fn read_constant_long(&mut self) -> Value {
        let low = self.read_byte();
        let medium = self.read_byte();
        let high = self.read_byte();
        let idx = ((high as usize) << 16) + ((medium as usize) << 8) + low as usize;
        self.chunk.value_array().values()[idx].clone()
    }
    fn runtime_error(&self, format: &str) {
        eprintln!("{}", format);
        let instruction = self.ip - 1;
        let line = self.chunk.lines().get_line(instruction as u32).unwrap();
        eprintln!("[line {}] in script", line)
        //todo reset the stack
    }
}
