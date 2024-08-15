#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Registers {
    A,
    B,
    C,
    D,
    SP,
    PC,
    BP,
    Flags,
}

impl Registers {
    pub fn from_u8_custom(value: u8) -> Option<Self> {
        match value {
            0 => Some(Registers::A),
            1 => Some(Registers::B),
            2 => Some(Registers::C),
            3 => Some(Registers::D),
            4 => Some(Registers::SP),
            5 => Some(Registers::PC),
            6 => Some(Registers::BP),
            7 => Some(Registers::Flags),
            _ => None,
        }
    }

    pub fn from_str_custom(value: &str) -> Option<Self> {
        match value {
            "A" => Some(Registers::A),
            "B" => Some(Registers::B),
            "C" => Some(Registers::C),
            "D" => Some(Registers::D),
            "SP" => Some(Registers::SP),
            "PC" => Some(Registers::PC),
            "BP" => Some(Registers::BP),
            "Flags" => Some(Registers::Flags),
            _ => None,
        }
    }
}
