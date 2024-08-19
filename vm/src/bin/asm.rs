use anyhow::Result;

use std::{
    env,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
};

use vm::{Instruction, Registers};

#[derive(Debug)]
enum EncodedInstruction {
    SingleByte(u8),
    TwoBytes(u8, u8),
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
            "AddRegister" => {
                let reg1 = parts.get(1).ok_or_else(|| {
                    anyhow::anyhow!("AddRegister instruction requires two registers")
                })?;
                let reg2 = parts.get(2).ok_or_else(|| {
                    anyhow::anyhow!("AddRegister instruction requires two registers")
                })?;

                let reg1 = Registers::from_str_custom(reg1).ok_or_else(|| {
                    anyhow::anyhow!("Invalid first register for AddRegister instruction")
                })?;
                let reg2 = Registers::from_str_custom(reg2).ok_or_else(|| {
                    anyhow::anyhow!("Invalid second register for AddRegister instruction")
                })?;

                if (reg1 as u8) > 3 || (reg2 as u8) > 3 {
                    return Err(anyhow::anyhow!(
                        "you dumbass, you can also specify registers A, B, C, D in AddRegister"
                    ));
                }
                Ok(Instruction::AddRegister(reg1, reg2))
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
                let opcode = 0x50;
                EncodedInstruction::SingleByte(opcode | ((*register as u8) & 0x0F))
            }
            Instruction::AddStack => EncodedInstruction::SingleByte(0x30),
            Instruction::AddRegister(reg1, reg2) => {
                let opcode = 0x40;
                EncodedInstruction::SingleByte(
                    (opcode | ((*reg1 as u8) & 0x03) << 2) | ((*reg2 as u8) & 0x03),
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
        }
    }
    stdout
        .write_all(&bytes)
        .map_err(|x| anyhow::anyhow!("{}", x))?;
    Ok(())
}
