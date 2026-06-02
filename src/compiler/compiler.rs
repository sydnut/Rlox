use crate::chunk::Chunk;
use crate::chunk::OpCode::*;
use crate::compiler::scanner::Scanner;
use crate::compiler::token::{Token, TokenType};

struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    scanner: Scanner<'a>,
    chunk: Chunk,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner<'a>) -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            scanner,
            chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
        }
    }

    /// 向前读取一个 token，跳过 error token
    fn advance(&mut self) {
        self.previous = self.current;
        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }
            // error token 的 start 字段就是错误信息
            self.error_at_current(self.current.start);
        }
    }

    /// 消费当前 token，如果不是期望的类型则报错
    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    /// 发出单字节字节码
    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte, self.previous.line as u32);
    }

    /// 发出常量指令
    fn emit_constant(&mut self, value: f64) {
        self.chunk.write_constant(value, self.previous.line as u32);
    }

    /// 结束编译，发出 OP_RETURN
    fn end_compiler(&mut self) {
        self.emit_byte(u8::from(OpReturn));
        if !self.had_error {
            self.chunk.disassemble("code");
        }
    }

    /// 在 current token 位置报错
    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    /// 在 previous token 位置报错
    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    /// 报错核心实现
    fn error_at(&mut self, token: Token<'a>, message: &str) {
        // 紧急模式下抑制后续错误
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);
        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => {}
            _ => eprint!(" at '{}'", &token.start[..token.length]),
        }
        eprintln!(": {}", message);
        self.had_error = true;
    }
}

// ============================================================
// Binding Power — Pratt 解析器的优先级核心
// ============================================================

/// 中缀运算符的 binding power: (left_bp, right_bp)
/// - 左结合: left < right（如 +: (6, 7)），右操作数"吸力"更大
/// - 右结合: left > right（如 =: (2, 1)），左操作数"吸力"更大
/// - 返回 (0, 0) 表示该 token 不是中缀运算符
fn infix_binding_power(token_type: TokenType) -> (u8, u8) {
    match token_type {
        // 赋值（右结合）— 后续章节启用
        TokenType::Equal => (2, 1),
        // 相等性
        TokenType::EqualEqual | TokenType::BangEqual => (4, 5),
        // 比较
        TokenType::Less | TokenType::LessEqual
        | TokenType::Greater | TokenType::GreaterEqual => (5, 6),
        // 加减
        TokenType::Plus | TokenType::Minus => (6, 7),
        // 乘除
        TokenType::Star | TokenType::Slash => (7, 8),
        // 不是中缀运算符
        _ => (0, 0),
    }
}

/// 前缀运算符的 binding power
/// 返回 0 表示该 token 不是前缀运算符
fn prefix_binding_power(token_type: TokenType) -> u8 {
    match token_type {
        TokenType::Minus => 8, // 一元取负
        TokenType::Bang => 8,  // 逻辑非 — 后续章节启用
        _ => 0,
    }
}

// ============================================================
// Pratt 解析器
// ============================================================

/// 解析入口：解析一个完整的表达式
fn expression(parser: &mut Parser) {
    parse_expression(parser, 0);
}

/// Pratt 解析核心
///
/// `min_bp` 是当前上下文允许的最低优先级：
/// - 只有 binding power >= min_bp 的中缀运算符才会被处理
/// - 调用者通过控制 min_bp 来限制解析范围
fn parse_expression(parser: &mut Parser, min_bp: u8) {
    // === 第一阶段：解析前缀表达式 ===
    parser.advance();
    match parser.previous.token_type {
        TokenType::Number => number(parser),
        TokenType::LeftParen => grouping(parser),
        TokenType::Minus => unary(parser),
        _ => {
            parser.error("Expect expression.");
            return; // 遇到错误后直接返回，不继续解析
        }
    }

    // === 第二阶段：循环处理中缀运算符 ===
    loop {
        let (l_bp, r_bp) = infix_binding_power(parser.current.token_type);
        // l_bp == 0 → 不是中缀运算符，退出循环
        // l_bp < min_bp → 优先级不够，退出循环
        if l_bp == 0 || l_bp < min_bp {
            break;
        }
        parser.advance(); // 消费中缀运算符
        binary(parser, r_bp);
    }
}

// ============================================================
// 各表达式类型的解析函数
// ============================================================

/// 数字字面量：将词素转为 f64 并发出常量指令
fn number(parser: &mut Parser) {
    let value: f64 = parser.previous.start[..parser.previous.length]
        .parse()
        .unwrap();
    parser.emit_constant(value);
}

/// 括号分组：递归解析内部表达式，然后消费右括号
fn grouping(parser: &mut Parser) {
    parse_expression(parser, 0);
    parser.consume(TokenType::RightParen, "Expect ')' after expression.");
}

/// 一元运算符（前缀）：先解析操作数，再发出运算指令
fn unary(parser: &mut Parser) {
    let op_type = parser.previous.token_type;
    // 用一元运算符自身的 binding power 限制操作数的优先级
    // 这样 -a + b 只会解析为 (-a) + b，而不是 -(a + b)
    parse_expression(parser, prefix_binding_power(op_type));
    match op_type {
        TokenType::Minus => parser.emit_byte(u8::from(OpNegate)),
        _ => unreachable!(),
    }
}

/// 二元运算符（中缀）：先解析右操作数，再发出运算指令
///
/// `r_bp` 是右操作数的 binding power：
/// - 左结合运算符的 r_bp > l_bp，使右操作数"更贪心"
/// - 右结合运算符的 r_bp < l_bp，使左操作数"更贪心"
fn binary(parser: &mut Parser, r_bp: u8) {
    let op_type = parser.previous.token_type;
    // 用 r_bp 解析右操作数
    parse_expression(parser, r_bp);
    match op_type {
        TokenType::Plus => parser.emit_byte(u8::from(OpAdd)),
        TokenType::Minus => parser.emit_byte(u8::from(OpSub)),
        TokenType::Star => parser.emit_byte(u8::from(OpMul)),
        TokenType::Slash => parser.emit_byte(u8::from(OpDiv)),
        _ => unreachable!(),
    }
}

// ============================================================
// 编译入口
// ============================================================

/// 编译源代码，成功返回包含字节码的 Chunk，失败返回 None
pub fn compile(source: &str) -> Option<Chunk> {
    let mut parser = Parser::new(Scanner::new(source));
    parser.advance(); // "启动泵"—— 将cur->Token#1
    expression(&mut parser);
    parser.consume(TokenType::Eof, "Expect end of expression.");
    parser.end_compiler();
    if parser.had_error {
        None
    } else {
        Some(parser.chunk)
    }
}
