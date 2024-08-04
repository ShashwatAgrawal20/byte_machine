#![allow(dead_code)]

use anyhow::Result;
mod memory;
mod vm;
use crate::vm::Machine;

fn main() -> Result<()> {
    let mut vm = Machine::new();
    vm.step()
}
