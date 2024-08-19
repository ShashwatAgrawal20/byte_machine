#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Registers {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl Registers {
    pub fn from_u8_custom(value: u8) -> Option<Self> {
        match value {
            0 => Some(Registers::A),
            1 => Some(Registers::B),
            2 => Some(Registers::C),
            3 => Some(Registers::D),
            4 => Some(Registers::E),
            5 => Some(Registers::F),
            6 => Some(Registers::H),
            7 => Some(Registers::L),
            _ => None,
        }
    }

    pub fn from_str_custom(value: &str) -> Option<Self> {
        match value {
            "A" => Some(Registers::A),
            "B" => Some(Registers::B),
            "C" => Some(Registers::C),
            "D" => Some(Registers::D),
            "E" => Some(Registers::E),
            "F" => Some(Registers::F),
            "H" => Some(Registers::H),
            "L" => Some(Registers::L),
            _ => None,
        }
    }
}
