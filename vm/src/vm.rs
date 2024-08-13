use anyhow::Result;
use std::collections::HashMap;

use crate::memory::Memory;

type Interrupt = fn(&mut Machine) -> Result<()>;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Registers {
    A,
    B,
    C,
    D,
    SP,
    PC,
    BP,
    Flags,
}

impl Registers {
    fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(Registers::A),
            1 => Some(Registers::B),
            2 => Some(Registers::C),
            3 => Some(Registers::D),
            4 => Some(Registers::SP),
            5 => Some(Registers::PC),
            6 => Some(Registers::BP),
            7 => Some(Registers::Flags),
            _ => None,
        }
    }

    pub fn from_str_custom(value: &str) -> Option<Self> {
        match value {
            "A" => Some(Registers::A),
            "B" => Some(Registers::B),
            "C" => Some(Registers::C),
            "D" => Some(Registers::D),
            "SP" => Some(Registers::SP),
            "PC" => Some(Registers::PC),
            "BP" => Some(Registers::BP),
            "Flags" => Some(Registers::Flags),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopRegister(Registers),
    AddStack,
    AddRegister(Registers, Registers),
    Interrupt(u8),
}

pub struct Machine {
    pub registers: [u8; 8],
    pub halt: bool,
    pub memory: Memory,
    interrupts: HashMap<u8, Interrupt>,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            halt: false,
            interrupts: HashMap::new(),
            memory: Memory::new(0xff),
        }
    }

    pub fn define_interrupt(&mut self, index: u8, f: Interrupt) {
        self.interrupts.insert(index, f);
    }

    pub fn step(&mut self) -> Result<()> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        println!(
            "PC -> {:?}   |   OPCODE -> 0x{:X}   |   INST -> {:?}",
            self.registers[Registers::PC as usize],
            opcode >> 4,
            instruction,
        );

        let _ = match instruction {
            Instruction::Nop => Ok(()),
            Instruction::Push(value) => self.push(value),
            Instruction::PopRegister(r) => {
                let value = self.pop()?;
                self.registers[r as usize] = value;
                Ok(())
            }
            Instruction::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a.wrapping_add(b))
            }
            Instruction::AddRegister(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            }
            Instruction::Interrupt(signal) => {
                let signal_function = self.interrupts.get(&signal).ok_or(anyhow::anyhow!(
                    "0x{:X} is not a valid signal, dumbass!",
                    signal
                ))?;
                signal_function(self)
            }
        };
        Ok(())
    }

    fn push(&mut self, v: u8) -> Result<()> {
        let sp = self.registers[Registers::SP as usize];
        if self.memory.write(sp, v).is_err() {
            return Err(anyhow::anyhow!("memory write fault @ 0x{:X}", sp));
        }
        self.registers[Registers::SP as usize] += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<u8> {
        let sp = self.registers[Registers::SP as usize] - 1;
        if let Ok(v) = self.memory.read(sp) {
            self.registers[Registers::SP as usize] -= 1;
            Ok(v)
        } else {
            Err(anyhow::anyhow!("memory read fault @ 0x{:X}", sp))
        }
    }

    fn fetch(&mut self) -> Result<u8> {
        let pc = self.registers[Registers::PC as usize];
        let opcode = self.memory.read(pc)?;
        self.registers[Registers::PC as usize] = pc.wrapping_add(1);
        Ok(opcode)
    }

    fn decode(&mut self, opcode: u8) -> Result<Instruction> {
        let args = opcode & 0x0F;
        match opcode >> 4 {
            0x0 => Ok(Instruction::Nop),
            0x1 => {
                let value = self.fetch()?;
                Ok(Instruction::Push(value))
            }
            0x2 => match Registers::from(args) {
                Some(reg) => Ok(Instruction::PopRegister(reg)),
                None => Err(anyhow::anyhow!("Invalid register code: {}", args)),
            },
            0x3 => Ok(Instruction::AddStack),
            0x4 => {
                let reg1 = Registers::from(args >> 2)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args >> 2))?;
                let reg2 = Registers::from(args & 0x03)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args & 0x03))?;
                Ok(Instruction::AddRegister(reg1, reg2))
            }
            0xF => Ok(Instruction::Interrupt(args)),
            _ => Err(anyhow::anyhow!("Unknown opcode: {:X}", opcode)),
        }
    }
}
