use nom::IResult;

use crate::{ir::Statement, opcodes::Opcode};

mod bytecode;
mod error;
mod opcodes;

pub use bytecode::bytecode_file_parser;
pub use error::ParserError;

pub type ParserResult<'a, O> = IResult<&'a [u8], O, ParserError>;

pub trait OpcodeStatement: Sized {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement>;
}
