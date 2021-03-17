use nom::{
    combinator::map,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::{terminated, tuple},
};

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
    Delete {
        object: Register,
    },
    Get {
        object: Register,
    },
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
    obj_type: Object,
    property: Property,
    kind: ObjectExpKind,
}

impl ObjectExpression {
    fn new(obj_type: Object, property: Property, kind: ObjectExpKind) -> Self {
        Self {
            obj_type,
            property,
            kind,
        }
    }

    fn parse_string_get(object: Object, opcode: Opcode, input: &[u8]) -> ParserResult<Self> {
        let (input, kind) = map(terminated(le_u8, le_u8), |byte| ObjectExpKind::Get {
            object: Register::Byte(byte),
        })(input)?;

        use Opcode::*;
        let (input, string_index) = match opcode {
            GetByIdShort => map(le_u8, StringIndex::Byte)(input),
            GetById | TryGetById => map(le_u16, StringIndex::Word)(input),
            GetByIdLong | TryGetByIdLong => map(le_u32, StringIndex::Dword)(input),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a GetBy Object Expression", opcode),
            ))?,
        }?;

        let property = Property::String(string_index);
        Ok((input, Self::new(object, property, kind)))
    }

    fn parse_string_set(
        object: Object,
        opcode: Opcode,
        enumerable: bool,
        input: &[u8],
    ) -> ParserResult<Self> {
        let (input, kind) = map(terminated(le_u8, le_u8), |byte| ObjectExpKind::Set {
            value: Register::Byte(byte),
            enumerable,
        })(input)?;

        use Opcode::*;
        let (input, string_index) = match opcode {
            PutNewOwnByIdShort => map(le_u8, StringIndex::Byte)(input),
            PutById | TryPutById | PutNewOwnById | PutNewOwnNEById => {
                map(le_u16, StringIndex::Word)(input)
            }
            PutByIdLong | TryPutByIdLong | PutNewOwnByIdLong | PutNewOwnNEByIdLong => {
                map(le_u32, StringIndex::Dword)(input)
            }
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a PutBy Object Expression", opcode),
            ))?,
        }?;
        let property = Property::String(string_index);

        Ok((input, Self::new(object, property, kind)))
    }

    fn parse_string_delete(is_long: bool, input: &[u8]) -> ParserResult<Self> {
        let (input, kind) = map(le_u8, |byte| ObjectExpKind::Delete {
            object: Register::Byte(byte),
        })(input)?;

        let (input, string_index) = match is_long {
            true => map(le_u32, StringIndex::Dword)(input),
            false => map(le_u16, StringIndex::Word)(input),
        }?;
        let property = Property::String(string_index);

        Ok((input, Self::new(Object::Normal, property, kind)))
    }

    fn parse_put_own_index(is_long: bool, input: &[u8]) -> ParserResult<Self> {
        let (input, value) = map(le_u8, Register::Byte)(input)?;
        let kind = ObjectExpKind::Set {
            value,
            enumerable: true,
        };

        let (input, array_index) = match is_long {
            true => map(le_u32, ArrayIndex::Dword)(input),
            false => map(le_u8, ArrayIndex::Byte)(input),
        }?;
        let property = Property::Index(array_index);

        Ok((input, Self::new(Object::Array, property, kind)))
    }

    fn parse_put_own_val(input: &[u8]) -> ParserResult<Self> {
        map(
            tuple((le_u8, le_u8, le_u8)),
            |(value_byte, property_byte, enumerable_byte)| {
                let property = Property::Register(Register::Byte(property_byte));

                let kind = ObjectExpKind::Set {
                    value: Register::Byte(value_byte),
                    enumerable: enumerable_byte != 0,
                };

                Self::new(Object::Normal, property, kind)
            },
        )(input)
    }

    fn parse_by_val(opcode: Opcode, input: &[u8]) -> ParserResult<Self> {
        let (input, (reg_2, reg_3)) = map(tuple((le_u8, le_u8)), |(byte_1, byte_2)| {
            (Register::Byte(byte_1), Register::Byte(byte_2))
        })(input)?;

        let object_expression = match opcode {
            Opcode::GetByVal => {
                let kind = ObjectExpKind::Get { object: reg_2 };
                let property = Property::Register(reg_3);
                Self::new(Object::Normal, property, kind)
            }
            Opcode::PutByVal => {
                let kind = ObjectExpKind::Set {
                    value: reg_3,
                    enumerable: true,
                };
                let property = Property::Register(reg_2);
                Self::new(Object::Normal, property, kind)
            }
            Opcode::DelByVal => {
                let kind = ObjectExpKind::Delete { object: reg_2 };
                let property = Property::Register(reg_3);
                Self::new(Object::Normal, property, kind)
            }
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a ByVal Object Expression ", opcode),
            ))?,
        };

        Ok((input, object_expression))
    }

    fn parse_define(input: &[u8]) -> ParserResult<Self> {
        map(
            tuple((le_u8, le_u8, le_u8, le_u8)),
            |(property_byte, getter_byte, setter_byte, enumerable_byte)| {
                let property = Property::Register(Register::Byte(property_byte));

                let kind = ObjectExpKind::Define {
                    getter: Register::Byte(getter_byte),
                    setter: Register::Byte(setter_byte),
                    enumerable: enumerable_byte != 0,
                };

                Self::new(Object::Normal, property, kind)
            },
        )(input)
    }
}

impl OpcodeStatement for ObjectExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, register) = map(le_u8, Register::Byte)(input)?;

        use Opcode::*;
        let (input, object_expression) = match opcode {
            GetByIdShort | GetById | GetByIdLong => {
                Self::parse_string_get(Object::Normal, opcode, input)
            }
            TryGetById | TryGetByIdLong => Self::parse_string_get(Object::Global, opcode, input),

            PutById | PutByIdLong | PutNewOwnById | PutNewOwnByIdShort | PutNewOwnByIdLong => {
                Self::parse_string_set(Object::Normal, opcode, true, input)
            }
            PutNewOwnNEById | PutNewOwnNEByIdLong => {
                Self::parse_string_set(Object::Normal, opcode, false, input)
            }
            TryPutById | TryPutByIdLong => {
                Self::parse_string_set(Object::Global, opcode, true, input)
            }

            PutOwnByIndex => Self::parse_put_own_index(false, input),
            PutOwnByIndexL => Self::parse_put_own_index(true, input),

            PutOwnByVal => Self::parse_put_own_val(input),

            DelById => Self::parse_string_delete(false, input),
            DelByIdLong => Self::parse_string_delete(true, input),

            GetByVal | PutByVal | DelByVal => Self::parse_by_val(opcode, input),

            PutOwnGetterSetterByVal => Self::parse_define(input),

            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not an Object Expression", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register,
            expression: Expression::ObjExp(object_expression),
        };
        Ok((input, statement))
    }
}
