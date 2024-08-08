use anyhow::{Context, Result};
mod memory;
mod vm;
use vm::{Machine, Registers};

use std::{env, fs};

fn main() -> Result<()> {
    let mut vm = Machine::new();

    let bytes: Vec<u8> = fs::read_to_string(
        env::args()
            .nth(1)
            .context("where's the program file you dumbass!")?,
    )
    .context("can't read the file, try giving relative path from crate root.")?
    .split_whitespace()
    .filter_map(|s| {
        if s.starts_with("0x") {
            u8::from_str_radix(&s[2..], 16).ok()
        } else {
            s.parse().ok()
        }
    })
    .collect();

    println!("{:?}", bytes);
    vm.memory.load(&bytes)?;
    while vm.registers[Registers::PC as usize] != bytes.len() as u8 {
        vm.step()?
    }
    println!("reg A = {}", vm.registers[Registers::A as usize]);

    Ok(())
}
