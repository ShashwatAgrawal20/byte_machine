use anyhow::Result;

use crate::memory::Memory;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Push(u8),
    PopRegister(Registers),
    AddStack,
    AddRegister(Registers, Registers),
}

pub struct Machine {
    pub registers: [u8; 8],
    pub memory: Memory,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            memory: Memory::new(0xff),
        }
    }

    pub fn step(&mut self) -> Result<()> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        println!(
            "PC -> {:?}   |   OPCODE -> 0x{:X}   |   INST -> {:?}",
            self.registers[Registers::PC as usize],
            opcode,
            instruction
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
        };
        Ok(())
    }

    pub fn get_register(&self, r: Registers) -> u8 {
        self.registers[r as usize]
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
        match opcode {
            0x00 => Ok(Instruction::Nop),
            0x01 => {
                let value = self.fetch()?;
                Ok(Instruction::Push(value))
            }
            0x02 => {
                let reg_code = self.fetch()?;
                match Registers::from(reg_code) {
                    Some(reg) => Ok(Instruction::PopRegister(reg)),
                    None => Err(anyhow::anyhow!("Invalid register code: {}", reg_code)),
                }
            }
            0x03 => Ok(Instruction::AddStack),
            0x04 => {
                let reg_pair = self.fetch()?;
                let reg1 = Registers::from((reg_pair >> 4) & 0x0F)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", reg_pair >> 4))?;
                let reg2 = Registers::from(reg_pair & 0x0F).ok_or(anyhow::anyhow!(
                    "Invalid register code: {}",
                    reg_pair & 0x0F
                ))?;
                Ok(Instruction::AddRegister(reg1, reg2))
            }
            _ => Err(anyhow::anyhow!("Unknown opcode: {:X}", opcode)),
        }
    }
}
