use crate::Machine;
use anyhow::Result;

pub fn halt_interrupt(vm: &mut Machine) -> Result<()> {
    vm.halt = true;
    Ok(())
}
