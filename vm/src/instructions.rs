use crate::Registers;

// #[derive(Debug)]
// pub enum Instruction {
//     Nop,
//     Push(u8),
//     PopRegister(Registers),
//     PushRegister(Registers),
//     AddStack,
//     AddRegister(Registers, Registers),
//     Interrupt(u8),
// }
//

#[derive(Debug)]
pub enum Instruction {
    Nop,                                        // 0000 0000
    Push(u8),                                   // 0001 xxxx | iiiiiiii
    PopRegister(Registers),                     // 0010 rrrr
    AddStack,                                   // 0011 0000
    AddRegister(Registers, Registers),          // 0100 rr |  rr
    PushRegister(Registers),                    // 0101 rrrr
    LoadImmediate(Registers, u8),               // 0110 rrrr | iiiiiiii
    LoadMemory(Registers, u16),                 // 0111 rrrr | aaaaaaaa | aaaaaaaa
    // ALU(ALUOperation, Registers, Registers), // 1000 oooo | rrrr | rrrr
    // Jump(u16),                               // 1001 0000 | aaaaaaaa | aaaaaaaa
    // JumpConditional(JumpCondition, u16),     // 1010 cccc | aaaaaaaa | aaaaaaaa
    // Call(u16),                               // 1011 0000 | aaaaaaaa | aaaaaaaa
    // Return,                                  // 1100 0000
    Interrupt(u8),                              // 1111 iiii
}

#[derive(Debug)]
pub enum ALUOperation {
    Add,        // 0000
    Sub,        // 0001
    And,        // 0010
    Or,         // 0011
    Xor,        // 0100
    Not,        // 0101
    ShiftLeft,  // 0110
    ShiftRight, // 0111
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
