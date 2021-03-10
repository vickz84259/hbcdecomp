use nom::{
    combinator::map,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::{preceded, tuple},
};

use super::{Expression, Register, Statement, StringIndex};

use crate::{
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug)]
pub enum BufferIndex {
    Word(u16),
    Dword(u32),
}

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

#[derive(Debug)]
pub struct NewArrayExpression {
    array_size: u16,
    array_index: Option<BufferIndex>,
}

impl NewArrayExpression {
    fn new(array_size: u16) -> Self {
        Self {
            array_size,
            array_index: None,
        }
    }

    fn new_buffer(array_size: u16, array_index: u16) -> Self {
        Self {
            array_size,
            array_index: Some(BufferIndex::Word(array_index)),
        }
    }

    fn new_buffer_long(array_size: u16, array_index: u32) -> Self {
        Self {
            array_size,
            array_index: Some(BufferIndex::Dword(array_index)),
        }
    }
}

impl OpcodeStatement for NewArrayExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (remaining, (register_byte, array_size)) = tuple((le_u8, le_u16))(input)?;

        let (remaining, new_array) = match opcode {
            Opcode::NewArray => Ok((remaining, Self::new(array_size))),
            Opcode::NewArrayWithBuffer => {
                let (remaining, array_index) = le_u16(remaining)?;
                Ok((remaining, Self::new_buffer(array_size, array_index)))
            }
            Opcode::NewArrayWithBufferLong => {
                let (remaining, array_index) = le_u32(remaining)?;
                Ok((remaining, Self::new_buffer_long(array_size, array_index)))
            }
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a NewArrayExpression", opcode),
            )),
        }?;

        let statement = Statement::Expression {
            register: Register::Byte(register_byte),
            expression: Expression::NewArray(new_array),
        };

        Ok((remaining, statement))
    }
}

#[derive(Debug)]
pub enum EnvIndex {
    Byte(u8),
    Word(u16),
}

#[derive(Debug)]
pub struct EnvExpression {
    index: EnvIndex,
    value: Option<Register>,
}

#[derive(Debug)]
pub enum ArrayIndex {
    Byte(u8),
    Dword(u32),
}

#[derive(Debug)]
pub enum Property {
    String(StringIndex),
    Index(ArrayIndex),
    Register(Register),
}

#[derive(Debug)]
pub enum ObjectExpKind {
    Delete,
    Get,
    Set {
        value: Register,
        enumerable: bool,
    },
    Define {
        getter: Register,
        setter: Register,
        enumerable: bool,
    },
}

#[derive(Debug)]
pub enum Object {
    Normal,
    Array,
    Global,
    This,
    Environment { id: EnvIndex },
}

#[derive(Debug)]
pub struct ObjectExpression {
    object: Register,
    obj_type: Object,
    property: Property,
    kind: ObjectExpKind,
}
