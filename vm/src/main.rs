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
        vm.memory.write(0, 0x1)?;
        vm.memory.write(1, 60)?;
        vm.memory.write(2, 0x1)?;
        vm.memory.write(3, 9)?;
        vm.memory.write(4, 0x3)?;
        vm.memory.write(5, 0x2)?;
        vm.memory.write(6, 0)?;
        vm.step()?;
        vm.step()?;
        vm.step()?;
        vm.step()?;
        println!("reg A = {}", vm.get_register(Registers::A));
    }

    // AddRegister(A, B)
    // {
    //     println!("new program start & setting up the PC to 0");
    //     vm.registers[Registers::PC as usize] = 0;
    //     vm.registers[Registers::A as usize] = 10;
    //     vm.registers[Registers::B as usize] = 10;
    //     vm.memory.write(0, 0x4)?;
    //     vm.memory.write(1, 0x01)?;
    //     println!("reg A -> {}", vm.get_register(Registers::A));
    //     println!("reg B -> {}", vm.get_register(Registers::B));
    //     vm.step()?;
    //     println!("reg A = {}", vm.get_register(Registers::A));
    // }
    Ok(())
}
