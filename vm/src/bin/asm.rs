use anyhow::Result;

use std::{
    env,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
};

use vm::{ALUOperation, Instruction, Registers};

#[derive(Debug)]
enum EncodedInstruction {
    SingleByte(u8),
    TwoBytes(u8, u8),
    ThreeBytes(u8, u8, u8),
}

trait LocalToAsm {
    fn from(instruction: Vec<&str>) -> Result<Self>
    where
        Self: Sized;
    fn encode_u8(&self) -> EncodedInstruction;
}

impl LocalToAsm for Instruction {
    fn from(parts: Vec<&str>) -> Result<Self> {
        match *parts
            .first()
            .ok_or_else(|| anyhow::anyhow!("where's the instruction you dumbass!"))?
        {
            "Nop" => Ok(Instruction::Nop),
            "Push" => {
                let value = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Push instruction requires a value"))?
                    .parse::<u8>()
                    .map_err(|_| anyhow::anyhow!("Invalid value for Push instruction"))?;
                Ok(Instruction::Push(value))
            }
            "PopRegister" => {
                let register = parts.get(1).ok_or_else(|| {
                    anyhow::anyhow!("PopRegister instruction requires a register")
                })?;
                let reg = Registers::from_str_custom(register).ok_or_else(|| {
                    anyhow::anyhow!("Invalid register for PopRegister instruction")
                })?;
                Ok(Instruction::PopRegister(reg))
            }
            "PushRegister" => {
                let register = parts.get(1).ok_or_else(|| {
                    anyhow::anyhow!("PopRegister instruction requires a register")
                })?;
                let reg = Registers::from_str_custom(register).ok_or_else(|| {
                    anyhow::anyhow!("Invalid register for PopRegister instruction")
                })?;
                Ok(Instruction::PushRegister(reg))
            }
            "AddStack" => Ok(Instruction::AddStack),
            "LoadImmediate" => {
                let reg = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("LoadImmediate instruction requires a register"))
                    .and_then(|reg_str| {
                        Registers::from_str_custom(reg_str).ok_or_else(|| {
                            anyhow::anyhow!("Invalid register for LoadImmediate instruction")
                        })
                    })?;

                let value = parts
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("Push instruction requires a value"))?
                    .parse::<u8>()
                    .map_err(|_| anyhow::anyhow!("LoadImmediate expect a u8"))?;

                Ok(Instruction::LoadImmediate(reg, value))
            }
            "LoadMemory" => {
                let reg = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("LoadMemory instruction requires a register"))
                    .and_then(|reg_str| {
                        Registers::from_str_custom(reg_str).ok_or_else(|| {
                            anyhow::anyhow!("Invalid register for AddRegister instruction")
                        })
                    })?;

                let memory_str = parts.get(2).ok_or_else(|| {
                    anyhow::anyhow!("LoadMemory instruction requires a memory address")
                })?;
                let memory = u16::from_str_radix(
                    memory_str.strip_prefix("0x").unwrap_or(memory_str),
                    if memory_str.starts_with("0x") { 16 } else { 10 },
                )
                .map_err(|_| anyhow::anyhow!("Invalid address for LoadImmediate instruction"))?;

                Ok(Instruction::LoadMemory(reg, memory))
            }
            "Store" => {
                let reg = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Store instruction requires a register"))
                    .and_then(|reg_str| {
                        Registers::from_str_custom(reg_str).ok_or_else(|| {
                            anyhow::anyhow!("Invalid register for Store instruction")
                        })
                    })?;

                let memory_str = parts.get(2).ok_or_else(|| {
                    anyhow::anyhow!("Store instruction requires a memory address")
                })?;
                let memory = u16::from_str_radix(
                    memory_str.strip_prefix("0x").unwrap_or(memory_str),
                    if memory_str.starts_with("0x") { 16 } else { 10 },
                )
                .map_err(|_| anyhow::anyhow!("Invalid address for Store instruction"))?;

                // println!("{:?}, {}", reg, memory);

                Ok(Instruction::Store(reg, memory))
            }
            "ALU" => {
                let operation = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("ALU instruction requires a operation"))
                    .and_then(|operation_str| {
                        ALUOperation::from_str_custom(operation_str)
                            .ok_or_else(|| anyhow::anyhow!("Invalid Operation for ALU"))
                    })?;

                let reg1 = parts
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("Where's the 1st register dawg?"))
                    .and_then(|reg_str| {
                        Registers::from_str_custom(reg_str).ok_or_else(|| {
                            anyhow::anyhow!("Invalid first register for ALU instruction")
                        })
                    })?;
                let reg2 = parts
                    .get(3)
                    .ok_or_else(|| anyhow::anyhow!("Where's the 2nd register dawg?"))
                    .and_then(|reg_str| {
                        Registers::from_str_custom(reg_str).ok_or_else(|| {
                            anyhow::anyhow!("Invalid second register for ALU instruction")
                        })
                    })?;

                // println!("{:?}, {:?}, {:?}", operation, reg1, reg2);

                Ok(Instruction::ALU(operation, reg1, reg2))
            }
            "Interrupt" => {
                let value = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Interrupt instruction requires a value"))?
                    .parse::<u8>()
                    .map_err(|_| anyhow::anyhow!("Invalid value for interrupt instruction"))?;
                Ok(Instruction::Interrupt(value))
            }
            err => Err(anyhow::anyhow!(
                "You're a piece of shit.(from Assmeber), unexpected '{}'",
                err
            )),
        }
    }

    fn encode_u8(&self) -> EncodedInstruction {
        match self {
            Instruction::Nop => EncodedInstruction::SingleByte(0x00),
            Instruction::Push(value) => {
                let opcode = 0x10;
                EncodedInstruction::TwoBytes(opcode, *value)
            }
            Instruction::PopRegister(register) => {
                let opcode = 0x20;
                EncodedInstruction::SingleByte(opcode | ((*register as u8) & 0x0F))
            }
            Instruction::PushRegister(register) => {
                let opcode = 0x30;
                EncodedInstruction::SingleByte(opcode | ((*register as u8) & 0x0F))
            }
            Instruction::AddStack => EncodedInstruction::SingleByte(0x40),
            Instruction::LoadImmediate(reg, value) => {
                let opcode = 0x50;
                EncodedInstruction::TwoBytes(opcode | ((*reg as u8) & 0x0F), *value)
            }
            Instruction::LoadMemory(reg, address) => {
                let opcode = 0x60;

                EncodedInstruction::ThreeBytes(
                    opcode | ((*reg as u8) & 0x0F),
                    (address >> 8) as u8,
                    (address & 0x00FF) as u8,
                )
            }
            Instruction::Store(reg, address) => {
                let opcode = 0x70;
                EncodedInstruction::ThreeBytes(
                    opcode | ((*reg as u8) & 0x0F),
                    (address >> 8) as u8,
                    (address & 0x00FF) as u8,
                )
            }
            Instruction::ALU(operation, reg1, reg2) => {
                let opcode = 0x80;
                EncodedInstruction::TwoBytes(
                    opcode | ((*operation as u8) & 0xF),
                    ((*reg1 as u8) << 4) | (*reg2 as u8),
                )
            }
            Instruction::Interrupt(value) => {
                let opcode = 0xF0;
                EncodedInstruction::SingleByte(opcode | *value)
            }
        }
    }
}

fn main() -> Result<()> {
    let file =
        File::open(Path::new(&env::args().nth(1).ok_or_else(|| {
            anyhow::anyhow!("where's the program file you dumbass!")
        })?))
        .map_err(|_| anyhow::anyhow!("can't open the file, try giving a valid path."))?;

    let mut stdout = stdout().lock();
    let mut bytes: Vec<u8> = Vec::new();
    for line in BufReader::new(&file).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let result = <Instruction as LocalToAsm>::from(parts)?;
        let encoded = <Instruction as LocalToAsm>::encode_u8(&result);
        // println!("result = {:?}  |  encoded = {:?}", result, encoded);
        match encoded {
            EncodedInstruction::SingleByte(byte) => {
                bytes.push(byte);
            }
            EncodedInstruction::TwoBytes(byte1, byte2) => {
                bytes.push(byte1);
                bytes.push(byte2);
            }
            EncodedInstruction::ThreeBytes(byte1, byte2, byte3) => {
                bytes.push(byte1);
                bytes.push(byte2);
                bytes.push(byte3);
            }
        }
    }
    stdout
        .write_all(&bytes)
        .map_err(|x| anyhow::anyhow!("{}", x))?;
    Ok(())
}
