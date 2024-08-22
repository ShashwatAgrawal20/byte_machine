use anyhow::Result;
use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use vm::interrupts::halt_interrupt;
use vm::{Machine, Registers};

fn main() -> Result<()> {
    let mut vm = Machine::new();

    vm.memory.write(0xfffe, 69)?;
    //
    // vm.memory.write(0, 0x70)?;
    // vm.memory.write(1, 0xff)?;
    // vm.memory.write(2, 0xfe)?;
    // vm.memory.write(3, 0xff)?;
    // vm.step()?;

    let file =
        File::open(Path::new(&env::args().nth(1).ok_or_else(|| {
            anyhow::anyhow!("where's the program file you dumbass!")
        })?))
        .map_err(|_| anyhow::anyhow!("can't open the file, try giving a valid path."))?;

    let mut bytes: Vec<u8> = Vec::new();
    BufReader::new(file).read_to_end(&mut bytes)?;

    println!(
        "[{}]",
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    );
    vm.define_interrupt(0xF, halt_interrupt);
    vm.memory.load(&bytes)?;
    if bytes.is_empty() {
        return Err(anyhow::anyhow!("empty binary"));
    }
    while !vm.halt {
        println!("{}", vm.state());
        vm.step()?
    }
    println!("reg A = {}", vm.registers[Registers::A as usize]);
    Ok(())
}
