use nom::{
    combinator::map,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::tuple,
};

use super::BufferIndex;

use crate::{
    ir::{Expression, Register, Statement},
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug)]
pub struct NewArrayExpression {
    array_size: u16,
    array_index: Option<BufferIndex>,
}

impl NewArrayExpression {
    fn parse_new(input: &[u8], array_size: u16) -> ParserResult<Self> {
        let new_array = Self {
            array_size,
            array_index: None,
        };
        Ok((input, new_array))
    }

    fn parse_buffer(input: &[u8], array_size: u16, is_long: bool) -> ParserResult<Self> {
        let (input, array_index) = match is_long {
            true => map(le_u32, |x| Some(BufferIndex::Dword(x)))(input),
            false => map(le_u16, |x| Some(BufferIndex::Word(x)))(input),
        }?;

        let new_array = Self {
            array_size,
            array_index,
        };
        Ok((input, new_array))
    }
}

impl OpcodeStatement for NewArrayExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, (register_byte, array_size)) = tuple((le_u8, le_u16))(input)?;

        let (input, new_array) = match opcode {
            Opcode::NewArray => Self::parse_new(input, array_size),
            Opcode::NewArrayWithBuffer => Self::parse_buffer(input, array_size, false),
            Opcode::NewArrayWithBufferLong => Self::parse_buffer(input, array_size, true),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a NewArrayExpression", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register: Register::Byte(register_byte),
            expression: Expression::NewArray(new_array),
        };

        Ok((input, statement))
    }
}
