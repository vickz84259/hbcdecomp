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
    fn parse_mov(input: &[u8], is_long: bool) -> ParserResult<Statement> {
        let (input, (left_register, right_register)) = if is_long {
            map(tuple((le_u32, le_u32)), |(left, right)| {
                (Register::Dword(left), Register::Dword(right))
            })(input)
        } else {
            map(tuple((le_u8, le_u8)), |(left, right)| {
                (Register::Byte(left), Register::Byte(right))
            })(input)
        }?;

        let statement = Statement::Expression {
            register: left_register,
            expression: Expression::Register(right_register),
        };
        Ok((input, statement))
    }
}

impl OpcodeStatement for Register {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        match opcode {
            Opcode::Mov => Self::parse_mov(input, false),
            Opcode::MovLong => Self::parse_mov(input, true),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a Mov opcode", opcode),
            ))?,
        }
    }
}
