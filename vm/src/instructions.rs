use crate::Registers;

#[derive(Debug)]
pub enum Instruction {
    Nop,                                     // 0000 0000
    Push(u8),                                // 0001 xxxx | iiiiiiii
    PopRegister(Registers),                  // 0010 rrrr
    PushRegister(Registers),                 // 0011 rrrr
    AddStack,                                // 0100 0000
    LoadImmediate(Registers, u8),            // 0101 rrrr | iiiiiiii
    LoadMemory(Registers, u16),              // 0110 rrrr | aaaaaaaa | aaaaaaaa
    Store(Registers, u16),                   // 0111 rrrr | aaaaaaaa | aaaaaaaa
    ALU(ALUOperation, Registers, Registers), // 1000 oooo | rrrr | rrrr
    // Jump(u16),                               // 1001 0000 | aaaaaaaa | aaaaaaaa
    // JumpConditional(JumpCondition, u16),     // 1010 cccc | aaaaaaaa | aaaaaaaa
    // Call(u16),                               // 1011 0000 | aaaaaaaa | aaaaaaaa
    // Return,                                  // 1100 0000
    Interrupt(u8), // 1111 iiii
}

#[derive(Debug, Copy, Clone)]
pub enum ALUOperation {
    Add, // 0000
    Sub, // 0001
    Mul, // 0010
    Div, // 0011
}

#[derive(Debug)]
pub enum JumpCondition {
    Always,   // 0000
    Zero,     // 0001
    NotZero,  // 0010
    Carry,    // 0011
    NotCarry, // 0100
    Negative, // 0101
    Positive, // 0110
    Overflow, // 0111
}

impl ALUOperation {
    pub fn from_u8_custom(value: u8) -> Option<Self> {
        match value {
            0 => Some(ALUOperation::Add),
            1 => Some(ALUOperation::Sub),
            2 => Some(ALUOperation::Mul),
            3 => Some(ALUOperation::Div),
            _ => None,
        }
    }

    pub fn from_str_custom(value: &str) -> Option<Self> {
        match value {
            "Add" => Some(ALUOperation::Add),
            "Sub" => Some(ALUOperation::Sub),
            "Mul" => Some(ALUOperation::Mul),
            "Div" => Some(ALUOperation::Div),
            _ => None,
        }
    }
}
