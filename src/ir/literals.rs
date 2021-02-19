#[derive(Debug, Copy, Clone)]
pub enum Number {
    UInt(u8),
    Int(i32),
    Double(f64),
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Boolean {
    True,
    False,
}

impl From<Boolean> for &'static str {
    fn from(value: Boolean) -> Self {
        use Boolean::*;

        match value {
            True => "true",
            False => "false",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RegExp {
    pattern_index: u32,
    flag_index: u32,
    bytecode_index: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum StringIndex {
    Byte(u8),
    Word(u16),
    Dword(u32),
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    String(StringIndex),
    Number(Number),
    Boolean(Boolean),
    RegExp(RegExp),
    Null,
    Undefined,
}
