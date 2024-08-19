use anyhow::Result;
use std::collections::HashMap;

use crate::{instructions::Instruction, memory::Memory, registers::Registers};

type Interrupt = fn(&mut Machine) -> Result<()>;

pub struct Machine {
    pub registers: [u8; 8],
    pub halt: bool,
    pub memory: Memory,
    pub pc: u16,
    pub sp: u16,
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
            pc: 0,
            sp: 0,
            memory: Memory::new(0xffff),
        }
    }

    pub fn state(&self) -> String {
        format!(
            "A: {} | B: {} | C: {} | D: {} E: {} | F: {} | H: {} L: {:X}",
            self.get_register(Registers::A),
            self.get_register(Registers::B),
            self.get_register(Registers::C),
            self.get_register(Registers::D),
            self.get_register(Registers::E),
            self.get_register(Registers::F),
            self.get_register(Registers::H),
            self.get_register(Registers::L)
        )
    }

    pub fn define_interrupt(&mut self, index: u8, f: Interrupt) {
        self.interrupts.insert(index, f);
    }

    pub fn get_register(&self, r: Registers) -> u8 {
        self.registers[r as usize]
    }

    pub fn set_register(&mut self, r: Registers, v: u8) {
        self.registers[r as usize] = v;
    }

    pub fn step(&mut self) -> Result<()> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        println!(
            "PC -> {:?}   |   OPCODE -> 0x{:X}   |   INST -> {:?}",
            // self.registers[Registers::PC as usize],
            self.pc,
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
            Instruction::PushRegister(r) => {
                self.push(self.registers[r as usize])?;
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
        // let sp = self.registers[Registers::SP as usize];
        let sp = self.sp;
        // println!("sp = {sp}");
        if self.memory.write(sp, v).is_err() {
            return Err(anyhow::anyhow!("memory write fault @ 0x{:X}", sp));
        }
        // self.registers[Registers::SP as usize] += 1;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<u8> {
        // let sp = self.registers[Registers::SP as usize] - 1;
        // let sp = self.sp - 1;
        let sp = self.sp.wrapping_sub(1);
        if let Ok(v) = self.memory.read(sp) {
            // self.registers[Registers::SP as usize] -= 1;
            self.sp = self.sp.wrapping_sub(1);
            Ok(v)
        } else {
            Err(anyhow::anyhow!("memory read fault @ 0x{:X}", sp))
        }
    }

    fn fetch(&mut self) -> Result<u8> {
        // let pc = self.registers[Registers::PC as usize];
        let opcode = self.memory.read(self.pc)?;
        // self.registers[Registers::PC as usize] = pc.wrapping_add(1);
        self.pc = self.pc.wrapping_add(1);
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
            0x2 => match Registers::from_u8_custom(args) {
                Some(reg) => Ok(Instruction::PopRegister(reg)),
                None => Err(anyhow::anyhow!("Invalid register code: {}", args)),
            },
            0x3 => Ok(Instruction::AddStack),
            0x4 => {
                let reg1 = Registers::from_u8_custom(args >> 2)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args >> 2))?;
                let reg2 = Registers::from_u8_custom(args & 0x03)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args & 0x03))?;
                Ok(Instruction::AddRegister(reg1, reg2))
            }
            0x5 => match Registers::from_u8_custom(args) {
                Some(reg) => Ok(Instruction::PushRegister(reg)),
                None => Err(anyhow::anyhow!("Invalid register code: {}", args)),
            },
            0xF => Ok(Instruction::Interrupt(args)),
            _ => Err(anyhow::anyhow!("Unknown opcode: {:X}", opcode)),
        }
    }
}
