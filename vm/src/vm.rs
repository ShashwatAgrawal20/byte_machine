use anyhow::Result;
use std::fmt;

use crate::memory::Memory;

enum Registers {
    A,
    B,
    C,
    D,
    SP,
    PC,
    BP,
    FLAGS,
}

#[derive(Debug)]
pub enum Instruction {
    NOP,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::NOP => write!(f, "NOP"),
        }
    }
}

pub struct Machine {
    registers: [u8; 8],
    memory: Memory,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Memory::new(0xff),
        }
    }

    pub fn step(&mut self) -> Result<()> {
        let pc = self.registers[Registers::PC as usize];
        let opcode = self.memory.read(pc)?;
        self.registers[Registers::PC as usize] = pc.wrapping_add(1);
        let instruction = self.decode(opcode)?;
        println!("PC -> {} OPCODE -> {} INST -> {}", pc, opcode, instruction);
        Ok(())
    }

    fn decode(&self, opcode: u8) -> Result<Instruction> {
        match opcode {
            0x00 => Ok(Instruction::NOP),
            _ => Err(anyhow::anyhow!("Unknown opcode: {}", opcode)),
        }
    }
}
