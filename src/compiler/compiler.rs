use crate::chunk::OpCode::OpReturn;
use crate::chunk::obj::Object;
use crate::chunk::value::Value;
use crate::chunk::{Chunk, OpCode};
use crate::compiler;
use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenType};
use std::collections::HashMap;
use std::rc::Rc;
struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    scanner: Scanner<'a>,
    chunk: Chunk,
    pub had_error: bool,
    pub panic_mode: bool,
    //驻留表
    table: &'a mut HashMap<String, Rc<Object>>,
}
impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>, table: &'a mut HashMap<String, Rc<Object>>) -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            scanner,
            chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
            table,
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
    /// 匹配成功则消耗一个Token
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    /// current的Type是否是传入参数
    fn check(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }
    /// 生成字节码
    fn emit_byte(&mut self, byte: OpCode) {
        self.chunk
            .write_chunk(u8::from(byte), self.previous.line as u32);
    }
    /// 批量生成字节码
    fn emit_bytes(&mut self, bytes: &[OpCode]) {
        bytes.iter().for_each(|byte| self.emit_byte(*byte));
    }
    // 写入常量
    fn emit_content_number(&mut self, value: f64) {
        self.chunk
            .write_constant(Value::Double(value), self.previous.line as u32);
    }
    fn emit_content_str(&mut self, value: &str) {
        //查常量表，查不到新增，查到复用
        let str = self
            .table
            .entry(String::from(value))
            .or_insert(Rc::new(Object::String(String::from(value))));
        self.chunk
            .write_constant(Value::Obj(Rc::clone(str)), self.previous.line as u32);
    }
}
//Parse 辅助函数
impl Parser<'_> {
    fn parse_variable(&mut self, error_msg: &str) -> usize {
        self.consume(TokenType::Identifier, error_msg);
        // 把变量名写入常量池，返回索引
        let name = &self.previous.start[..self.previous.length];
        let obj = self
            .table
            .entry(String::from(name))
            .or_insert(Rc::new(Object::String(String::from(name))));
        self.chunk.add_constant(Value::Obj(Rc::clone(obj)))
    }
    fn define_variable(&mut self, global: usize) {
        let line = self.previous.line as u32;
        self.chunk.write_global_op(
            OpCode::OpDefineGlobal,
            OpCode::OpDefineGlobalLong,
            global,
            line,
        );
    }
    fn error_at_current(&mut self, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.error_at(&self.current, message);
        self.had_error = true;
    }
    fn error(&mut self, message: &str) {
        if self.panic_mode {
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
    fn synchronize(&mut self) {
        self.panic_mode = false;
        while self.current.token_type != TokenType::Eof {
            //步过;
            if self.previous.token_type == TokenType::Semicolon {
                return;
            }
            match self.current.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}
fn parse_expression(parser: &mut Parser, min_bp: u8) {
    //移动一格,使previous有值
    parser.advance();
    match parser.previous.token_type {
        TokenType::LeftParen => paren(parser),
        TokenType::Minus | TokenType::Bang => unary(parser),
        TokenType::Number
        | TokenType::True
        | TokenType::False
        | TokenType::Nil
        | TokenType::String => value(parser),
        TokenType::Identifier => identifier(parser, min_bp),
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
    if min_bp==0 && parser.match_token(TokenType::Equal){
        parser.error("Invalid assignment target.");
    }
}
fn value(parser: &mut Parser) {
    match parser.previous.token_type {
        TokenType::Number => {
            let value: f64 = parser.previous.start[..parser.previous.length]
                .parse()
                .unwrap();
            parser.emit_content_number(value);
        }
        TokenType::False => {
            parser.emit_byte(OpCode::OpFalse);
        }
        TokenType::Nil => {
            parser.emit_byte(OpCode::OpNil);
        }
        TokenType::True => {
            parser.emit_byte(OpCode::OpTrue);
        }
        TokenType::String => {
            //scanner没有提取出b'"',这里需要去除
            let str = &parser.previous.start[1..parser.previous.length - 1];
            parser.emit_content_str(str);
        }
        _ => unreachable!(),
    }
}
fn identifier(parser: &mut Parser, min_bp: u8) {
    let name = &parser.previous.start[..parser.previous.length];
    let obj = parser
        .table
        .entry(String::from(name))
        .or_insert(Rc::new(Object::String(String::from(name))));
    let idx = parser.chunk.add_constant(Value::Obj(Rc::clone(obj)));
    let line = parser.previous.line as u32;

    if min_bp == 0 && parser.match_token(TokenType::Equal) {
        // 赋值：编译右值，emit OpSetGlobal
        expression(parser);
        parser.chunk.write_global_op(
            OpCode::OpSetGlobal,
            OpCode::OpSetGlobalLong,
            idx,
            line,
        );
    } else {
        // 读变量：emit OpGetGlobal
        parser.chunk.write_global_op(
            OpCode::OpGetGlobal,
            OpCode::OpGetGlobalLong,
            idx,
            line,
        );
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
        _ => unreachable!(),
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
        TokenType::BangEqual => parser.emit_bytes(&[OpCode::OpEqual, OpCode::OpNot]),
        TokenType::Greater => parser.emit_byte(OpCode::OpGreater),
        TokenType::GreaterEqual => parser.emit_bytes(&[OpCode::OpLess, OpCode::OpNot]),
        TokenType::Less => parser.emit_byte(OpCode::OpLess),
        TokenType::LessEqual => parser.emit_bytes(&[OpCode::OpGreater, OpCode::OpNot]),
        _ => unreachable!(),
    }
}
/// 表达式解析驱动函数
fn expression(parser: &mut Parser) {
    parse_expression(parser, 0);
}
fn var_declaration(parser: &mut Parser) {
    let global = parser.parse_variable("Expect variable name.");
    if parser.match_token(TokenType::Equal){
        //赋值
        expression(parser);
    }else{
        //隐式初始化为nil
        parser.emit_byte(OpCode::OpNil);
    }
    parser.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");
    parser.define_variable(global);
}
fn expression_statement(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::Semicolon, "Expect ';' after expression.");
    parser.emit_byte(OpCode::OpPop)
}
fn print_statement(parser: &mut Parser) {
    expression(parser);
    parser.consume(TokenType::Semicolon, "Expect ';' after value.");
    parser.emit_byte(OpCode::OpPrint);
}
fn declaration(parser: &mut Parser) {
    if parser.match_token(TokenType::Var) {
        var_declaration(parser);
    } else {
        statement(parser);
    }
    if parser.panic_mode {
        //恐慌模式后,进行同步
        parser.synchronize()
    }
}
fn statement(parser: &mut Parser) {
    if parser.match_token(TokenType::Print) {
        print_statement(parser);
    } else {
        expression_statement(parser);
    }
}
// 以下为语法分析函数
pub fn compile(source: &str, table: &mut HashMap<String, Rc<Object>>) -> Option<Chunk> {
    let mut parser = Parser::new(Scanner::new(source), table);
    parser.advance(); //reset the `current` -> Token#1
    while !parser.match_token(TokenType::Eof) {
        declaration(&mut parser);
    }
    parser.end_compiler();
    if !parser.had_error {
        Some(parser.chunk)
    } else {
        None
    }
}
