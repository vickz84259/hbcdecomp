use super::Register;
use crate::builtins::Builtins;

#[derive(Debug)]
pub enum FunctionIndex {
    Register(Register),
    Word(u16),
    Dword(u32),
}

#[derive(Debug)]
pub enum ArgsNo {
    Byte(u8),
    Dword(u32),
}

#[derive(Debug)]
pub enum FunctionType {
    Normal,
    Constructor,
    BuiltIn(Builtins),
}

#[derive(Debug)]
pub struct FrameCall {
    function: FunctionIndex,
    no_of_arguments: ArgsNo,
    function_type: FunctionType,
}

#[derive(Debug)]
pub struct CallExpression {
    function: FunctionIndex,
    arguments: Vec<Register>,
}
