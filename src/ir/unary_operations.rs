use std::convert::TryFrom;

use nom::{number::complete::le_u8, sequence::tuple};

use super::{Expression, Register, Statement};

use crate::{
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug)]
pub enum UnaryOperator {
    Negation,
    LogicalNot,
    BitwiseNot,
    TypeOf,
}

impl From<UnaryOperator> for &'static str {
    fn from(operator: UnaryOperator) -> Self {
        use UnaryOperator::*;

        match operator {
            Negation => "-",
            LogicalNot => "!",
            BitwiseNot => "~",
            TypeOf => "typeof",
        }
    }
}

impl TryFrom<Opcode> for UnaryOperator {
    type Error = ParserError;

    fn try_from(opcode: Opcode) -> Result<Self, Self::Error> {
        match opcode {
            Opcode::Negate => Ok(Self::Negation),
            Opcode::Not => Ok(Self::LogicalNot),
            Opcode::BitNot => Ok(Self::BitwiseNot),
            Opcode::TypeOf => Ok(Self::TypeOf),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a valid Unary operation", opcode),
            )),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    operator: UnaryOperator,
    prefix: bool,
    argument: Register,
}

impl OpcodeStatement for UnaryExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (remaining, (byte, operand)) = tuple((le_u8, le_u8))(input)?;

        let expression = Expression::Unary(Self {
            operator: UnaryOperator::try_from(opcode)?,
            prefix: true,
            argument: Register::from(operand),
        });

        let statement = Statement::Expression {
            register: Register::from(byte),
            expression,
        };
        Ok((remaining, statement))
    }
}

#[derive(Debug)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

impl From<UpdateOperator> for &'static str {
    fn from(operator: UpdateOperator) -> Self {
        use UpdateOperator::*;

        match operator {
            Increment => "++",
            Decrement => "--",
        }
    }
}

#[derive(Debug)]
pub struct UpdateExpression {
    operator: UpdateOperator,
    prefix: bool,
    argument: Register,
}
