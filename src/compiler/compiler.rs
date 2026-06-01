use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenType};

pub fn compile(source: &str) {
    //todo initScanner
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:2} '{}'", token.token_type, &token.start[..token.length]);
        match token.token_type {
            TokenType::Eof => break,
            _ => {}
        }
    }
}
