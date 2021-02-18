#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Register {
    Byte(u8),
    Dword(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum StringIndex {
    Word(u16),
    Dword(u32),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Identifier {
    Register(Register),
    StringIndex(StringIndex),
}
