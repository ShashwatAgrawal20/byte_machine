use anyhow::Result;
mod memory;
mod vm;
use vm::{Machine, Registers};

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<()> {
    let mut vm = Machine::new();

    let file =
        File::open(Path::new(&env::args().nth(1).ok_or_else(|| {
            anyhow::anyhow!("where's the program file you dumbass!")
        })?))
        .map_err(|_| anyhow::anyhow!("can't open the file, try giving a valid path."))?;

    let mut bytes: Vec<u8> = Vec::new();
    for line in BufReader::new(&file).lines() {
        for token in line?.split_whitespace() {
            bytes.push(
                u8::from_str_radix(token, 16).map_err(|x| anyhow::anyhow!("parse fail: {}", x))?,
            );
        }
    }

    println!(
        "[{}]",
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    );
    vm.memory.load(&bytes)?;
    while !vm.halt {
        vm.step()?
    }
    println!("reg A = {}", vm.registers[Registers::A as usize]);
    Ok(())
}
