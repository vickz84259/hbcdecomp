use nom::{
    combinator::map,
    number::complete::{le_f64, le_i32, le_u16, le_u32, le_u8},
    sequence::tuple,
};

use crate::{
    ir::{Expression, Register, Statement},
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug, Copy, Clone)]
pub enum Number {
    UInt(u8),
    Int(i32),
    Double(f64),
}

impl Number {
    fn parse_number(opcode: Opcode, input: &[u8]) -> ParserResult<Literal> {
        let (input, number) = match opcode {
            Opcode::LoadConstZero => Ok((input, Number::UInt(0))),
            Opcode::LoadConstUInt8 => map(le_u8, Number::UInt)(input),
            Opcode::LoadConstInt => map(le_i32, Number::Int)(input),
            Opcode::LoadConstDouble => map(le_f64, Number::Double)(input),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a LoadConst (Number)", opcode),
            ))?,
        }?;
        Ok((input, Literal::Number(number)))
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Boolean {
    True,
    False,
}

impl From<Boolean> for &'static str {
    fn from(value: Boolean) -> Self {
        use Boolean::*;

        match value {
            True => "true",
            False => "false",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RegExp {
    pattern_index: u32,
    flag_index: u32,
    bytecode_index: u32,
}

impl RegExp {
    fn parse_regex(input: &[u8]) -> ParserResult<Literal> {
        map(
            tuple((le_u32, le_u32, le_u32)),
            |(pattern_index, flag_index, bytecode_index)| {
                Literal::RegExp(Self {
                    pattern_index,
                    flag_index,
                    bytecode_index,
                })
            },
        )(input)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StringIndex {
    Byte(u8),
    Word(u16),
    Dword(u32),
}

impl StringIndex {
    fn parse_index(input: &[u8], is_long: bool) -> ParserResult<Literal> {
        let (input, string_index) = match is_long {
            true => map(le_u32, StringIndex::Dword)(input),
            false => map(le_u16, StringIndex::Word)(input),
        }?;
        Ok((input, Literal::String(string_index)))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    String(StringIndex),
    Number(Number),
    Boolean(Boolean),
    RegExp(RegExp),
    Null,
    Undefined,
}

impl OpcodeStatement for Literal {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, register) = map(le_u8, Register::Byte)(input)?;

        use Opcode::*;
        let (input, literal) = match opcode {
            LoadConstZero | LoadConstUInt8 | LoadConstInt | LoadConstDouble => {
                Number::parse_number(opcode, input)
            }

            LoadConstString => StringIndex::parse_index(input, false),
            LoadConstStringLongIndex => StringIndex::parse_index(input, true),

            LoadConstTrue => Ok((input, Literal::Boolean(Boolean::True))),
            LoadConstFalse => Ok((input, Literal::Boolean(Boolean::False))),

            LoadConstUndefined => Ok((input, Literal::Undefined)),
            LoadConstNull => Ok((input, Literal::Null)),

            CreateRegExp => RegExp::parse_regex(input),

            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a Literal Expression", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register,
            expression: Expression::Literal(literal),
        };
        Ok((input, statement))
    }
}
