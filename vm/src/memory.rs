use anyhow::Result;

pub struct Memory {
    bytes: Vec<u8>,
    size: u16,
}

impl Memory {
    pub fn new(size: u16) -> Self {
        Self {
            bytes: vec![0; size.into()],
            size,
        }
    }

    pub fn read(&self, addr: u16) -> Result<u8> {
        if addr < self.size {
            Ok(self.bytes[addr as usize])
        } else {
            Err(anyhow::anyhow!("address {} out of bounds", addr))
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        if addr < self.size {
            self.bytes[addr as usize] = value;
            Ok(())
        } else {
            Err(anyhow::anyhow!("address {} out of bounds", addr))
        }
    }

    pub fn load(&mut self, program_vec: &[u8]) -> Result<()> {
        for (index, byte) in program_vec.iter().enumerate() {
            self.write(index as u16, *byte)?
        }
        Ok(())
    }
}
