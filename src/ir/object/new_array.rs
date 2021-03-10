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

    fn parse_buffer(input: &[u8], array_size: u16) -> ParserResult<Self> {
        map(le_u16, |array_index| Self {
            array_size,
            array_index: Some(BufferIndex::Word(array_index)),
        })(input)
    }

    fn parse_buffer_long(input: &[u8], array_size: u16) -> ParserResult<Self> {
        map(le_u32, |array_index| Self {
            array_size,
            array_index: Some(BufferIndex::Dword(array_index)),
        })(input)
    }
}

impl OpcodeStatement for NewArrayExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, (register_byte, array_size)) = tuple((le_u8, le_u16))(input)?;

        let (input, new_array) = match opcode {
            Opcode::NewArray => Self::parse_new(input, array_size),
            Opcode::NewArrayWithBuffer => Self::parse_buffer(input, array_size),
            Opcode::NewArrayWithBufferLong => Self::parse_buffer_long(input, array_size),
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
