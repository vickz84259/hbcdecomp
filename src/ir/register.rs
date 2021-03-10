use nom::{
    combinator::map,
    number::complete::{le_u32, le_u8},
    sequence::tuple,
};

use super::{Expression, Statement};

use crate::{
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    Byte(u8),
    Dword(u32),
}

impl Register {
    fn parse_mov(input: &[u8]) -> ParserResult<Statement> {
        map(tuple((le_u8, le_u8)), |(left, right)| {
            Statement::Expression {
                register: Register::Byte(left),
                expression: Expression::Register(Register::Byte(right)),
            }
        })(input)
    }

    fn parse_mov_long(input: &[u8]) -> ParserResult<Statement> {
        map(tuple((le_u32, le_u32)), |(left, right)| {
            Statement::Expression {
                register: Register::Dword(left),
                expression: Expression::Register(Register::Dword(right)),
            }
        })(input)
    }
}

impl OpcodeStatement for Register {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        match opcode {
            Opcode::Mov => Self::parse_mov(input),
            Opcode::MovLong => Self::parse_mov_long(input),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a Mov opcode", opcode),
            ))?,
        }
    }
}
