use nom::{
    combinator::map,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::{preceded, tuple},
};

use super::BufferIndex;

use crate::{
    ir::{Expression, Register, Statement},
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug)]
pub struct NewObjectExpression {
    no_of_static_elements: u16,
    key_index: BufferIndex,
    value_index: BufferIndex,
    parent: Option<Register>,
}

impl NewObjectExpression {
    fn parse_new(input: &[u8]) -> ParserResult<Self> {
        let new_object = Self {
            no_of_static_elements: 0,
            key_index: BufferIndex::Word(0),
            value_index: BufferIndex::Word(0),
            parent: None,
        };
        Ok((input, new_object))
    }

    fn parse_parent(input: &[u8]) -> ParserResult<Self> {
        map(le_u8, |parent| Self {
            no_of_static_elements: 0,
            key_index: BufferIndex::Word(0),
            value_index: BufferIndex::Word(0),
            parent: Some(Register::Byte(parent)),
        })(input)
    }

    fn parse_buffer(input: &[u8]) -> ParserResult<Self> {
        map(
            preceded(le_u16, tuple((le_u16, le_u16, le_u16))),
            |(no_of_static_elements, key_index, value_index)| Self {
                no_of_static_elements,
                key_index: BufferIndex::Word(key_index),
                value_index: BufferIndex::Word(value_index),
                parent: None,
            },
        )(input)
    }

    fn parse_buffer_long(input: &[u8]) -> ParserResult<Self> {
        map(
            preceded(le_u16, tuple((le_u16, le_u32, le_u32))),
            |(no_of_static_elements, key_index, value_index)| Self {
                no_of_static_elements,
                key_index: BufferIndex::Dword(key_index),
                value_index: BufferIndex::Dword(value_index),
                parent: None,
            },
        )(input)
    }
}

impl OpcodeStatement for NewObjectExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, register_byte) = le_u8(input)?;

        let (input, new_object) = match opcode {
            Opcode::NewObject => Self::parse_new(input),
            Opcode::NewObjectWithBuffer => Self::parse_buffer(input),
            Opcode::NewObjectWithBufferLong => Self::parse_buffer_long(input),
            Opcode::NewObjectWithParent => Self::parse_parent(input),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a NewObjectExpression", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register: Register::Byte(register_byte),
            expression: Expression::NewObject(new_object),
        };

        Ok((input, statement))
    }
}
