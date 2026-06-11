#[repr(u8)]
#[derive(Copy, Clone)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpConstantLong = 2,
    OpNegate = 3,
    OpAdd = 4,
    OpSub = 5,
    OpMul = 6,
    OpDiv = 7,
    OpNil = 8,
    OpTrue = 9,
    OpFalse = 10,
    OpNot = 11,
    OpEqual = 12,
    OpGreater = 13,
    OpLess = 14,
    OpPrint = 15,
    OpPop = 16,
    OpDefineGlobal = 17,
    OpGetGlobal = 18,
    OpSetGlobal = 19,
    OpDefineGlobalLong = 20,
    OpGetGlobalLong = 21,
    OpSetGlobalLong = 22,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        match op {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpConstantLong => 2,
            OpCode::OpNegate => 3,
            OpCode::OpAdd => 4,
            OpCode::OpSub => 5,
            OpCode::OpMul => 6,
            OpCode::OpDiv => 7,
            OpCode::OpNil => 8,
            OpCode::OpTrue => 9,
            OpCode::OpFalse => 10,
            OpCode::OpNot => 11,
            OpCode::OpEqual => 12,
            OpCode::OpGreater => 13,
            OpCode::OpLess => 14,
            OpCode::OpPrint => 15,
            OpCode::OpPop => 16,
            OpCode::OpDefineGlobal => 17,
            OpCode::OpGetGlobal => 18,
            OpCode::OpSetGlobal => 19,
            OpCode::OpDefineGlobalLong => 20,
            OpCode::OpGetGlobalLong => 21,
            OpCode::OpSetGlobalLong => 22,
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpReturn),
            1 => Ok(OpCode::OpConstant),
            2 => Ok(OpCode::OpConstantLong),
            3 => Ok(OpCode::OpNegate),
            4 => Ok(OpCode::OpAdd),
            5 => Ok(OpCode::OpSub),
            6 => Ok(OpCode::OpMul),
            7 => Ok(OpCode::OpDiv),
            8 => Ok(OpCode::OpNil),
            9 => Ok(OpCode::OpTrue),
            10 => Ok(OpCode::OpFalse),
            11 => Ok(OpCode::OpNot),
            12 => Ok(OpCode::OpEqual),
            13 => Ok(OpCode::OpGreater),
            14 => Ok(OpCode::OpLess),
            15 => Ok(OpCode::OpPrint),
            16 => Ok(OpCode::OpPop),
            17 => Ok(OpCode::OpDefineGlobal),
            18 => Ok(OpCode::OpGetGlobal),
            19 => Ok(OpCode::OpSetGlobal),
            20 => Ok(OpCode::OpDefineGlobalLong),
            21 => Ok(OpCode::OpGetGlobalLong),
            22 => Ok(OpCode::OpSetGlobalLong),
            _ => Err(format!("Unknown opcode: {}", value)),
        }
    }
}
