use anyhow::Result;
use std::collections::HashMap;

use crate::{
    instructions::{ALUOperation, Instruction},
    memory::Memory,
    registers::{Flags, Registers},
};

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

#[allow(dead_code)]
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
            "A: {} | B: {} | C: {} | D: {} SP: {} | PC: {} | BP: {} Flags: {:X}\n",
            self.get_register(Registers::A),
            self.get_register(Registers::B),
            self.get_register(Registers::C),
            self.get_register(Registers::D),
            self.sp,
            self.pc,
            self.get_register(Registers::BP),
            self.get_register(Registers::Flags)
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
            "\nPC -> {:?}   |   OPCODE -> 0x{:X}   |   INST -> {:?}",
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
                let (result, overflow) = a.overflowing_add(b);
                self.push(result)?;
                self.set_flag(Flags::Overflow, overflow);
                Ok(())
            }
            Instruction::LoadImmediate(reg, value) => {
                self.set_register(reg, value);
                Ok(())
            }
            Instruction::LoadMemory(reg, memaddress) => {
                let value = self.memory.read(memaddress)?;
                self.set_register(reg, value);
                Ok(())
            }
            Instruction::Store(reg, memaddress) => {
                let value = self.get_register(reg);
                self.memory.write(memaddress, value)?;
                // println!("{:?}, {}", reg, self.memory.read(memaddress)?);
                Ok(())
            }
            Instruction::ALU(operation, reg1, reg2) => {
                let (result, overflow) = match operation {
                    ALUOperation::Add => self
                        .get_register(reg1)
                        .overflowing_add(self.get_register(reg2)),
                    ALUOperation::Sub => self
                        .get_register(reg1)
                        .overflowing_sub(self.get_register(reg2)),
                    ALUOperation::Mul => self
                        .get_register(reg1)
                        .overflowing_mul(self.get_register(reg2)),
                    ALUOperation::Div => self
                        .get_register(reg1)
                        .overflowing_div(self.get_register(reg2)),
                };

                // println!("overflow = {overflow}");

                self.set_register(reg1, result);
                self.set_flag(Flags::Zero, result == 0);
                self.set_flag(Flags::Overflow, overflow);

                Ok(())
            }
            Instruction::Interrupt(signal) => {
                let signal_function = self.interrupts.get(&signal).ok_or(anyhow::anyhow!(
                    "0x{:X} is not a valid signal, dumbass!",
                    signal
                ))?;
                signal_function(self)
            } // _ => todo!(),
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

    fn set_flag(&mut self, flag: Flags, condition: bool) {
        // println!("Registers::Flags = {}", self.get_register(Registers::Flags));

        if condition {
            self.set_register(
                Registers::Flags,
                self.get_register(Registers::Flags) | flag as u8,
            );
        } else {
            self.set_register(
                Registers::Flags,
                self.get_register(Registers::Flags) & !(flag as u8),
            );
        }
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        self.set_register(
            Registers::Flags,
            self.get_register(Registers::Flags) & !(flag as u8),
        );
    }

    fn is_flag_set(&self, flag: Flags) -> bool {
        self.get_register(Registers::Flags) & flag as u8 != 0
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
            0x3 => match Registers::from_u8_custom(args) {
                Some(reg) => Ok(Instruction::PushRegister(reg)),
                None => Err(anyhow::anyhow!("Invalid register code: {}", args)),
            },
            0x4 => Ok(Instruction::AddStack),
            // LoadImmediate(Register, value)
            // 0110 rrrr | iiiiiiii
            0x5 => {
                let reg = Registers::from_u8_custom(args)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args))?;
                let value = self.fetch()?;
                Ok(Instruction::LoadImmediate(reg, value))
            }
            // LoadMemory(Register, address)
            // 0111 rrrr | aaaaaaaa | aaaaaaaa
            0x6 => {
                let reg = Registers::from_u8_custom(args)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args))?;
                let value = (self.fetch()? as u16) << 8 | self.fetch()? as u16;

                // println!("{:02X}", value);
                Ok(Instruction::LoadMemory(reg, value))
            }
            // Store(Registers, u16),
            // 1000 rrrr | aaaaaaaa | aaaaaaaa
            0x7 => {
                let reg = Registers::from_u8_custom(args)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", args))?;
                let value = (self.fetch()? as u16) << 8 | self.fetch()? as u16;

                Ok(Instruction::Store(reg, value))
            }
            0x8 => {
                let operation = ALUOperation::from_u8_custom(args)
                    .ok_or(anyhow::anyhow!("Invalid operation code: {}", args))?;

                let next = self.fetch()?;
                let reg1 = Registers::from_u8_custom(next >> 4)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", next >> 4))?;

                let reg2 = Registers::from_u8_custom(next & 0xF)
                    .ok_or(anyhow::anyhow!("Invalid register code: {}", next & 0xF))?;

                Ok(Instruction::ALU(operation, reg1, reg2))
            }
            0xF => Ok(Instruction::Interrupt(args)),
            _ => Err(anyhow::anyhow!("Unknown opcode: {:X}", opcode)),
        }
    }
}
