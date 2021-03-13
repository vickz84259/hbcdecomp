use nom::{combinator::map, number::complete::le_u8};

use crate::{
    ir::{Expression, Register, Statement, StringIndex},
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

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
    Environment { id: u8 },
}

impl OpcodeStatement for Object {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, register) = map(le_u8, Register::Byte)(input)?;

        let (input, object) = match opcode {
            Opcode::GetEnvironment => map(le_u8, |id| Object::Environment { id })(input),
            Opcode::GetGlobalObject => Ok((input, Object::Global)),
            Opcode::CreateEnvironment => Ok((input, Object::Environment { id: 0 })),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not an Object", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register,
            expression: Expression::Object(object),
        };
        Ok((input, statement))
    }
}

#[derive(Debug)]
pub struct ObjectExpression {
    object: Register,
    obj_type: Object,
    property: Property,
    kind: ObjectExpKind,
}
