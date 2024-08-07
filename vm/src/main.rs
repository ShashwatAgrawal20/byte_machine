use anyhow::Result;
mod memory;
mod vm;
use vm::{Machine, Registers};

fn main() -> Result<()> {
    let mut vm = Machine::new();

    // Push(60)
    // Push(9)
    // AddStack
    // PopRegister(A)
    {
        vm.memory.write(0, 0x10)?;
        vm.memory.write(1, 60)?;
        vm.memory.write(2, 0x10)?;
        vm.memory.write(3, 9)?;
        vm.memory.write(4, 0x30)?;
        vm.memory.write(5, 0x20)?;
        vm.step()?;
        vm.step()?;
        vm.step()?;
        vm.step()?;
        println!("reg A = {}", vm.registers[Registers::A as usize]);
    }

    // AddRegister(A, B)
    // {
    //     vm.registers[Registers::A as usize] = 10;
    //     vm.registers[Registers::B as usize] = 10;
    //     vm.memory.write(0, 0x41)?;
    //     println!("reg A = {}", vm.registers[Registers::A as usize]);
    //     println!("reg B = {}", vm.registers[Registers::B as usize]);
    //     vm.step()?;
    //     println!("reg A = {}", vm.registers[Registers::A as usize]);
    // }
    Ok(())
}
