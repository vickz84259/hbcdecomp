use nom::IResult;

mod bytecode;

pub use bytecode::bytecode_file_parser;

type ParserResult<'a, O> = IResult<&'a [u8], O>;
