#[repr(u8)]
pub enum OpCode{
    OpReturn = 0,
    OpConstant = 1,
    OpConstantLong = 2,
    OpNegate = 3,
    OpAdd = 4,
    OpSub = 5,
    OpMul = 6,
    OpDiv = 7,
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
            _ => Err(format!("Unknown opcode: {}", value)),
        }
    }
}