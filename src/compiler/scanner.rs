use crate::compiler::token::{Token, TokenType};

pub struct Scanner<'a> {
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) line: i32,
    pub source: &'a [u8],
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: source.as_bytes(),
        }
    }
    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return Token::make_token(self, TokenType::Eof);
        }
        match self.advance() {
            //number
            b'0'..=b'9' => self.number(),
            //identifier
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier(),
            b'(' => Token::make_token(self, TokenType::LeftParen),
            b')' => Token::make_token(self, TokenType::RightParen),
            b'{' => Token::make_token(self, TokenType::LeftBrace),
            b'}' => Token::make_token(self, TokenType::RightBrace),
            b',' => Token::make_token(self, TokenType::Comma),
            b'.' => Token::make_token(self, TokenType::Dot),
            b'-' => Token::make_token(self, TokenType::Minus),
            b'+' => Token::make_token(self, TokenType::Plus),
            b'/' => Token::make_token(self, TokenType::Slash),
            b'*' => Token::make_token(self, TokenType::Star),
            b';' => Token::make_token(self, TokenType::Semicolon),
            b'!' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Token::make_token(self, token_type)
            }
            b'=' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Token::make_token(self, token_type)
            }
            b'<' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                Token::make_token(self, token_type)
            }
            b'>' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                Token::make_token(self, token_type)
            }
            b'"' => self.string(),

            _ => Token::error("Unexpected character.", self.line),
        }
    }
}
fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}
//私有辅助函数
impl<'a> Scanner<'a> {
    fn is_at_end(&self) -> bool {
        self.source[self.current] == b'\0'
    }
    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }
    fn peek_next(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }
    fn peek(&self) -> u8 {
        self.source[self.current]
    }
    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                b' ' | b'\t' | b'\r' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                //跳过注释
                b'/' => {
                    if self.peek_next() == b'/' {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }
    fn string(&mut self) -> Token<'a> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Token::error("Unterminated string.", self.line)
        } else {
            //get b'"',skip it
            self.advance();
            Token::make_token(self, TokenType::String)
        }
    }
    fn number(&mut self) -> Token<'a> {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            //consume '.'
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        Token::make_token(self, TokenType::Number)
    }
    fn identifier(&mut self) -> Token<'a> {
        //第一个进入的必定不是digit
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        let token_type = self.identifier_type();
        Token::make_token(self, token_type)
    }
    fn identifier_type(&mut self) -> TokenType {
        match self.source[self.start] {
            b'a' => self.check_keyword(TokenType::And, 1, 2, "nd"),
            b'c' => self.check_keyword(TokenType::Class, 1, 4, "lass"),
            b'e' => self.check_keyword(TokenType::Else, 1, 3, "lse"),
            b'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        b'a' => self.check_keyword(TokenType::False, 2, 3, "lse"),
                        b'o' => self.check_keyword(TokenType::For, 2, 1, "r"),
                        b'u' => self.check_keyword(TokenType::Fun, 2, 1, "n"),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            b'i' => self.check_keyword(TokenType::If, 1, 1, "f"),
            b'n' => self.check_keyword(TokenType::Nil, 1, 2, "il"),
            b'o' => self.check_keyword(TokenType::Or, 1, 1, "r"),
            b'p' => self.check_keyword(TokenType::Print, 1, 4, "rint"),
            b'r' => self.check_keyword(TokenType::Return, 1, 5, "eturn"),
            b's' => self.check_keyword(TokenType::Super, 1, 4, "uper"),
            b't' => {
                if self.current - self.start > 1 {
                    let c = self.source[self.start + 1];
                    return if c == b'h' {
                        self.check_keyword(TokenType::This, 2, 2, "is")
                    } else if c == b'r' {
                        self.check_keyword(TokenType::True, 2, 2, "ue")
                    } else {
                        TokenType::Identifier
                    };
                }
                TokenType::Identifier
            }
            b'v' => self.check_keyword(TokenType::Var, 1, 2, "ar"),
            b'w' => self.check_keyword(TokenType::While, 1, 4, "hile"),
            _ => TokenType::Identifier,
        }
    }
    fn check_keyword(
        &self,
        /*target*/ token_type: TokenType,
        start: usize,
        length: usize,
        rest: &str,
    ) -> TokenType {
        if self.current - self.start == start + length
            && self.source[self.start + start..self.start + start + length] == *rest.as_bytes()
        {
            token_type
        } else {
            TokenType::Identifier
        }
    }
}
