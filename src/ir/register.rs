#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    Byte(u8),
    Dword(u32),
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u32> for Register {
    fn from(value: u32) -> Self {
        Self::Dword(value)
    }
}
