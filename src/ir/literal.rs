use super::Identifier;

#[derive(Debug)]
pub enum Number {
    UInt(u8),
    Int(i32),
    Double(f64),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RegExp {
    pattern_index: u32,
    flag_index: u32,
    bytecode_index: u32,
}

#[derive(Debug)]
pub enum Literal {
    Identifier(Identifier),
    Null,
    Number(Number),
    Boolean(Boolean),
    RegExp(RegExp),
}
