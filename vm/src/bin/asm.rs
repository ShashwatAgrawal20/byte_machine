use anyhow::Result;

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
};

use vm::{ALUOperation, Instruction, JumpCondition, JumpTarget, Registers};

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
    fn encode_u8(&self) -> Result<EncodedInstruction>;
    fn size(&self) -> u8;
}

impl LocalToAsm for Instruction {
    fn size(&self) -> u8 {
        match self {
            Instruction::Nop => 1,
            Instruction::Push(_) => 2,
            Instruction::PopRegister(_) => 1,
            Instruction::PushRegister(_) => 1,
            Instruction::AddStack => 1,
            Instruction::LoadImmediate(_, _) => 2,
            Instruction::LoadMemory(_, _) => 3,
            Instruction::Store(_, _) => 3,
            Instruction::ALU(_, _, _) => 2,
            Instruction::Jump(_) => 3,
            Instruction::JumpConditional(_, _) => 3,
            Instruction::Interrupt(_) => 1,
        }
    }

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
            "Jump" => {
                let target = parts
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Jump instruction requires a target"))?;

                let jump_target = if let Ok(address) = u16::from_str_radix(
                    target.strip_prefix("0x").unwrap_or(target),
                    if target.starts_with("0x") { 16 } else { 10 },
                ) {
                    JumpTarget::Address(address)
                } else {
                    JumpTarget::Label(target.to_string())
                };
                Ok(Instruction::Jump(jump_target))
            }
            "JumpConditional" => {
                let condition = parts
                    .get(1)
                    .ok_or_else(|| {
                        anyhow::anyhow!("JumpConditional instruction requires a Condition")
                    })
                    .and_then(|conditional_str| {
                        JumpCondition::from_str_custom(conditional_str)
                            .ok_or_else(|| anyhow::anyhow!("Invalid Condition for JumpConditional"))
                    })?;

                let target = parts.get(2).ok_or_else(|| {
                    anyhow::anyhow!("JumpConditional instruction requires a target")
                })?;

                let jump_target = if let Ok(address) = u16::from_str_radix(
                    target.strip_prefix("0x").unwrap_or(target),
                    if target.starts_with("0x") { 16 } else { 10 },
                ) {
                    JumpTarget::Address(address)
                } else {
                    JumpTarget::Label(target.to_string())
                };

                Ok(Instruction::JumpConditional(condition, jump_target))
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

    fn encode_u8(&self) -> Result<EncodedInstruction> {
        match self {
            Instruction::Nop => Ok(EncodedInstruction::SingleByte(0x00)),
            Instruction::Push(value) => {
                let opcode = 0x10;
                Ok(EncodedInstruction::TwoBytes(opcode, *value))
            }
            Instruction::PopRegister(register) => {
                let opcode = 0x20;
                Ok(EncodedInstruction::SingleByte(
                    opcode | ((*register as u8) & 0x0F),
                ))
            }
            Instruction::PushRegister(register) => {
                let opcode = 0x30;
                Ok(EncodedInstruction::SingleByte(
                    opcode | ((*register as u8) & 0x0F),
                ))
            }
            Instruction::AddStack => Ok(EncodedInstruction::SingleByte(0x40)),
            Instruction::LoadImmediate(reg, value) => {
                let opcode = 0x50;
                Ok(EncodedInstruction::TwoBytes(
                    opcode | ((*reg as u8) & 0x0F),
                    *value,
                ))
            }
            Instruction::LoadMemory(reg, address) => {
                let opcode = 0x60;

                Ok(EncodedInstruction::ThreeBytes(
                    opcode | ((*reg as u8) & 0x0F),
                    (address >> 8) as u8,
                    (address & 0x00FF) as u8,
                ))
            }
            Instruction::Store(reg, address) => {
                let opcode = 0x70;
                Ok(EncodedInstruction::ThreeBytes(
                    opcode | ((*reg as u8) & 0x0F),
                    (address >> 8) as u8,
                    (address & 0x00FF) as u8,
                ))
            }
            Instruction::ALU(operation, reg1, reg2) => {
                let opcode = 0x80;
                Ok(EncodedInstruction::TwoBytes(
                    opcode | ((*operation as u8) & 0xF),
                    ((*reg1 as u8) << 4) | (*reg2 as u8),
                ))
            }
            Instruction::Jump(target) => {
                let opcode = 0x90;
                let address = match target {
                    JumpTarget::Address(addr) => *addr,
                    JumpTarget::Label(label) => {
                        return Err(anyhow::anyhow!(
                            "Unresolved label in Jump instruction = {label}"
                        ))
                    }
                };

                Ok(EncodedInstruction::ThreeBytes(
                    opcode,
                    (address >> 8) as u8,
                    (address & 0x00FF) as u8,
                ))
            }
            Instruction::JumpConditional(condition, target) => {
                let opcode = 0xA0;
                let address = match target {
                    JumpTarget::Address(addr) => *addr,
                    JumpTarget::Label(label) => {
                        return Err(anyhow::anyhow!(
                            "Unresolved label in JumpConditional instruction = {label}"
                        ))
                    }
                };

                Ok(EncodedInstruction::ThreeBytes(
                    opcode | ((*condition as u8) & 0xF),
                    (address >> 8) as u8,
                    (address & 0x00FF) as u8,
                ))
            }
            Instruction::Interrupt(value) => {
                let opcode = 0xF0;
                Ok(EncodedInstruction::SingleByte(opcode | *value))
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
    let lines: Vec<String> = BufReader::new(&file).lines().collect::<Result<_, _>>()?;

    let mut labels = HashMap::new();
    let mut current_address = 0u16;

    for line in &lines {
        // println!("{current_address}");
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if let Some(label) = line.strip_suffix(':') {
            labels.insert(label.to_string(), current_address);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let instruction = <Instruction as LocalToAsm>::from(parts)?;
            current_address += instruction.size() as u16;
        }
    }
    // println!("current address = {current_address}");

    let mut bytes: Vec<u8> = Vec::new();
    for line in &lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') || line.ends_with(':') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut result = <Instruction as LocalToAsm>::from(parts)?;

        if let Instruction::Jump(ref mut address) = result {
            match address {
                JumpTarget::Label(label) => {
                    if let Some(&label_address) = labels.get(&label.to_string()) {
                        *address = JumpTarget::Address(label_address);
                    }
                }
                JumpTarget::Address(addr) => *address = JumpTarget::Address(*addr),
            }
        }

        if let Instruction::JumpConditional(_, ref mut address) = result {
            match address {
                JumpTarget::Label(label) => {
                    if let Some(&label_address) = labels.get(&label.to_string()) {
                        *address = JumpTarget::Address(label_address);
                    }
                }
                JumpTarget::Address(addr) => *address = JumpTarget::Address(*addr),
            }
        }

        let encoded = <Instruction as LocalToAsm>::encode_u8(&result)?;
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
