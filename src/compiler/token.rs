use crate::compiler::scanner::Scanner;
use std::fmt::{Display, Formatter};
#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub start: &'a str,
    pub length: usize,
    pub line: i32,
}
impl<'a> Token<'a> {
    fn new(token_type: TokenType, start: &'a str, length: usize, line: i32) -> Self {
        Self {
            token_type,
            start,
            length,
            line,
        }
    }
    pub fn make_token(sc: &Scanner<'a>, t: TokenType) -> Self {
        unsafe {
            Self::new(
                t,
                str::from_utf8_unchecked(&sc.source[sc.start..]),
                sc.current - sc.start,
                sc.line,
            )
        }
    }
    pub fn error(str: &'_ str, line: i32) -> Token<'_> {
        Token::new(TokenType::Error, str, str.len(), line)
    }
    pub fn default() -> Self {
        Self::new(TokenType::Eof, "", 0, 0)
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens. 单字符词法
    //(
    LeftParen,
    //)
    RightParen,
    //{
    LeftBrace,
    //}
    RightBrace,
    //,
    Comma,
    //.
    Dot,
    //-
    Minus,
    //+
    Plus,
    //;
    Semicolon,
    // /
    Slash,
    //*
    Star,
    // One or two character tokens. 一或两字符词法
    //'!'
    Bang,
    //'!='
    BangEqual,
    //=
    Equal,
    //==
    EqualEqual,
    //>
    Greater,
    //>=
    GreaterEqual,
    //<
    Less,
    //<=
    LessEqual,
    // Literals. 字面量
    Identifier,
    String,
    Number,
    // Keywords. 关键字
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub fn infix_binding_power(token_type: TokenType) -> (u8, u8) {
    match token_type {
        // 相等性
        TokenType::EqualEqual | TokenType::BangEqual => (4, 5),
        // 比较
        TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => {
            (5, 6)
        }
        // 加减
        TokenType::Plus | TokenType::Minus => (6, 7),
        // 乘除
        TokenType::Star | TokenType::Slash => (7, 8),
        // 不是中缀运算符
        _ => (0, 0),
    }
}
pub fn prefix_binding_power(token_type: TokenType) -> u8 {
    match token_type {
        //右binding power最大,使-1+2 => -1 +2 not -(1+2)
        TokenType::Minus => 8, // 一元取负
        TokenType::Bang => 8,  // 逻辑非 — 后续章节启用
        _ => 0,
    }
}
