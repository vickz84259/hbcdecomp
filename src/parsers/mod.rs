use nom::IResult;

mod bytecode;
mod error;

pub use bytecode::bytecode_file_parser;
pub use error::ParserError;

pub type ParserResult<'a, O> = IResult<&'a [u8], O, ParserError>;
