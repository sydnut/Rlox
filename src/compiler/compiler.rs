use std::cell::RefCell;
use std::rc::Rc;
use crate::chunk::Chunk;
use crate::chunk::OpCode::OpReturn;
use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenType};
struct Parser<'a>{
    current:Rc<RefCell<Token<'a>>>,
    previous:Rc<RefCell<Token<'a>>>,
    scanner: Scanner<'a>,
    chunk:Chunk,
    pub had_error:bool,
    pub panic_mode:bool,
}
impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>,chunk: Chunk) -> Self {
        Self{
            current:Rc::new(RefCell::new(Token::default())),
            previous:Rc::new(RefCell::new(Token::default())),
            scanner,
            chunk,
            had_error:false,
            panic_mode:false,
        }
    }
    /// 结束编译器,发送OpReturn字节码
    pub fn end_compiler(&mut self) {
        self.emit_byte(u8::from(OpReturn));
    }
    fn advance(&mut self) {
        //not impl `Copy` trait
        self.previous = Rc::clone(&self.current);
        loop {
            let next = self.scanner.scan_token();
            self.current=Rc::new(RefCell::new(next));
            match self.current.borrow().token_type {
                TokenType::Error =>break,
                _=>{}
            }
            let message = self.current.borrow().start;
            self.error_at_current(message);
        }
    }
    fn consume(&mut self,token_type: TokenType,message:&str) {
        if self.current.borrow().token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
    /// 生成字节码
    fn emit_byte(&mut self,byte:u8) {
        self.chunk.write_chunk(byte,self.previous.borrow().line as u32);
    }
    fn emit_bytes(&mut self,bytes:&[u8]) {
        bytes.iter().for_each(|&byte| self.emit_byte(byte));
    }
}
//Parse 辅助函数
impl Parser<'_> {
    fn error_at_current(&mut self,message:&str){
        if !self.panic_mode{return;}
        self.panic_mode = true;
        self.error_at(&self.current.borrow(), message);
        self.had_error = true;
    }
    fn error(&mut self, message: &str) {
        if !self.panic_mode{return;}
        self.panic_mode = true;
        self.error_at(&self.previous.borrow(), message);
        self.had_error = true;
    }
    fn error_at(&self,token:&Token,message:&str){
        eprint!("[line {}] Error", token.line);
        match token.token_type {
            TokenType::Eof =>eprint!(" at end"),
            TokenType::Error => { /* nothing */ }
            _=>eprint!(" at '{}'",&token.start[..token.length]),
        }
        eprintln!(": {}", message);
    }
}
// 以下为语法分析函数
pub fn compile(source: &str,chunk:Chunk)->bool {
    let mut parser = Parser::new(Scanner::new(source),chunk);
    parser.advance();
    expression();
    parser.consume(TokenType::Eof,"Expect end of expression.");
    parser.end_compiler();
    !parser.had_error
}
fn number(){
    
}
