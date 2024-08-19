use crate::Registers;

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopRegister(Registers),
    PushRegister(Registers),
    AddStack,
    AddRegister(Registers, Registers),
    Interrupt(u8),
}
