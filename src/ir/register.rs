#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    Byte(u8),
    Dword(u32),
}
