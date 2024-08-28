use crate::Registers;

#[derive(Debug)]
pub enum Instruction {
    Nop,                                        // 0000 0000
    Push(u8),                                   // 0001 xxxx | iiiiiiii
    PopRegister(Registers),                     // 0010 rrrr
    PushRegister(Registers),                    // 0011 rrrr
    AddStack,                                   // 0100 0000
    LoadImmediate(Registers, u8),               // 0101 rrrr | iiiiiiii
    LoadMemory(Registers, u16),                 // 0110 rrrr | aaaaaaaa | aaaaaaaa
    Store(Registers, u16),                      // 0111 rrrr | aaaaaaaa | aaaaaaaa
    ALU(ALUOperation, Registers, Registers),    // 1000 oooo | rrrr | rrrr
    Jump(JumpTarget),                           // 1001 0000 | aaaaaaaa | aaaaaaaa
    JumpConditional(JumpCondition, JumpTarget), // 1010 cccc | aaaaaaaa | aaaaaaaa
    Interrupt(u8),                              // 1111 iiii
}

#[derive(Debug)]
pub enum JumpTarget {
    Address(u16),
    Label(String),
}

#[derive(Debug, Copy, Clone)]
pub enum ALUOperation {
    Add, // 0000
    Sub, // 0001
    Mul, // 0010
    Div, // 0011
}

#[derive(Debug, Copy, Clone)]
pub enum JumpCondition {
    LT,  // 0000
    GT,  // 0001
    EQ,  // 0010
    NEQ, // 0011
    GE,  // 0100
    LE,  // 0101
}

impl JumpCondition {
    pub fn from_u8_custom(value: u8) -> Option<Self> {
        match value {
            0 => Some(JumpCondition::LT),
            1 => Some(JumpCondition::GT),
            2 => Some(JumpCondition::EQ),
            3 => Some(JumpCondition::NEQ),
            4 => Some(JumpCondition::GE),
            5 => Some(JumpCondition::LE),
            _ => None,
        }
    }

    pub fn from_str_custom(value: &str) -> Option<Self> {
        match value {
            "LT" => Some(JumpCondition::LT),
            "GT" => Some(JumpCondition::GT),
            "EQ" => Some(JumpCondition::EQ),
            "NEQ" => Some(JumpCondition::NEQ),
            "GE" => Some(JumpCondition::GE),
            "LE" => Some(JumpCondition::LE),
            _ => None,
        }
    }
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
