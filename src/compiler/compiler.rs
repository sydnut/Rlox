use crate::chunk::OpCode::OpReturn;
use crate::chunk::{Chunk, OpCode};
use crate::chunk::value::Value::Double;
use crate::compiler;
use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenType};
struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    scanner: Scanner<'a>,
    chunk: Chunk,
    pub had_error: bool,
    pub panic_mode: bool,
}
impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            scanner,
            chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
        }
    }
    /// 结束编译器,发送OpReturn字节码
    pub fn end_compiler(&mut self) {
        self.emit_byte(OpReturn);
    }
    fn advance(&mut self) -> Token<'a> {
        //not impl `Copy` trait
        self.previous = self.current;
        loop {
            let next = self.scanner.scan_token();
            self.current = next;
            match self.current.token_type {
                TokenType::Error => {}
                _ => {
                    return self.current;
                }
            }
            let message = self.current.start;
            self.error_at_current(message);
        }
    }
    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
    /// 生成字节码
    fn emit_byte(&mut self, byte: OpCode) {
        self.chunk.write_chunk(u8::from(byte), self.previous.line as u32);
    }
    /// 批量生成字节码
    fn emit_bytes(&mut self, bytes: &[OpCode]) {
        bytes.iter().for_each(|byte| self.emit_byte(*byte));
    }
    // 写入常量
    fn emit_content(&mut self, value: f64) {
        self.chunk.write_constant(Double(value), self.previous.line as u32)
    }
}
//Parse 辅助函数
impl Parser<'_> {
    fn error_at_current(&mut self, message: &str) {
        if !self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.error_at(&self.current, message);
        self.had_error = true;
    }
    fn error(&mut self, message: &str) {
        if !self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.error_at(&self.previous, message);
        self.had_error = true;
    }
    fn error_at(&self, token: &Token, message: &str) {
        eprint!("[line {}] Error", token.line);
        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => { /* nothing */ }
            _ => eprint!(" at '{}'", &token.start[..token.length]),
        }
        eprintln!(": {}", message);
    }
}
/// 表达式解析驱动函数
fn expression(parser: &mut Parser) {
    parse_expression(parser, 0);
}
fn parse_expression(parser: &mut Parser, min_bp: u8) {
    //移动一格,使previous有值
    parser.advance();
    match parser.previous.token_type {
        TokenType::LeftParen => paren(parser),
        TokenType::Minus | TokenType::Bang => unary(parser),
        TokenType::Number | TokenType::True | TokenType::False | TokenType::Nil => value(parser),
        _ => parser.error("unsupported expression."),
    }
    loop {
        let (lop, rop) = compiler::token::infix_binding_power(parser.current.token_type);
        //是前缀运算符(或EOF)、或者下个运算符binding_power不如传入的运算符对右边的binding_power
        if lop == 0 || lop < min_bp {
            break;
        }
        //消费op
        parser.advance();
        //将中缀运算符写入chunk
        binary_op(parser, rop);
    }
}
fn value(parser: &mut Parser) {
    match parser.previous.token_type {
        TokenType::Number =>{
            let value: f64 = parser.previous.start[..parser.previous.length]
                .parse()
                .unwrap();
            parser.emit_content(value);
        }
        TokenType::False=>{
            parser.emit_byte(OpCode::OpFalse);
        }
        TokenType::Nil=>{
            parser.emit_byte(OpCode::OpNil);
        }
        TokenType::True=>{
            parser.emit_byte(OpCode::OpTrue);
        }
        _=>unreachable!()
    }

}
fn paren(parser: &mut Parser) {
    parse_expression(parser, 0);
    parser.consume(TokenType::RightParen, "Expect ')' after expression.");
}
fn unary(parser: &mut Parser) {
    let op = parser.previous.token_type;
    parse_expression(parser, compiler::token::prefix_binding_power(op));
    match op {
        TokenType::Minus => parser.emit_byte(OpCode::OpNegate),
        //待添加新的前缀运算符
        TokenType::Bang => parser.emit_byte(OpCode::OpNot),
        _=>unreachable!()
    }
}
fn binary_op(parser: &mut Parser, min_op: u8) {
    let op = parser.previous.token_type;
    //先写入右操作数,后序遍历
    parse_expression(parser, min_op);
    match op {
        TokenType::Plus => parser.emit_byte(OpCode::OpAdd),
        TokenType::Minus => parser.emit_byte(OpCode::OpSub),
        TokenType::Star => parser.emit_byte(OpCode::OpMul),
        TokenType::Slash => parser.emit_byte(OpCode::OpDiv),
        TokenType::EqualEqual => parser.emit_byte(OpCode::OpEqual),
        TokenType::BangEqual=>parser.emit_bytes(&[OpCode::OpEqual,OpCode::OpNot]),
        TokenType::Greater => parser.emit_byte(OpCode::OpGreater),
        TokenType::GreaterEqual => parser.emit_bytes(&[OpCode::OpLess, OpCode::OpNot]),
        TokenType::Less => parser.emit_byte(OpCode::OpLess),
        TokenType::LessEqual => parser.emit_bytes(&[OpCode::OpGreater, OpCode::OpNot]),
        _ => unreachable!(),
    }
}
// 以下为语法分析函数
pub fn compile(source: &str) -> Option<Chunk> {
    let mut parser = Parser::new(Scanner::new(source));
    parser.advance(); //reset the `current` -> Token#1
    expression(&mut parser);
    parser.consume(TokenType::Eof, "Expect end of expression.");
    parser.end_compiler();
    if !parser.had_error {
        Some(parser.chunk)
    } else {
        None
    }
}
