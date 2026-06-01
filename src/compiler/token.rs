use crate::compiler::scanner::Scanner;
use std::fmt::{Display, Formatter};

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
    pub fn make_token(sc: &'a Scanner, t: TokenType) -> Self {
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
}
#[derive(Debug)]
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
