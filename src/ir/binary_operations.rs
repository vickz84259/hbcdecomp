use std::convert::TryFrom;

use nom::{number::complete::le_u8, sequence::tuple};

use super::{Expression, Register, Statement};

use crate::{
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug)]
pub enum BinaryOperator {
    Equality,
    InEquality,
    Identity,
    NonIdentity,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    RightShift,
    LeftShift,
    UnsignedRightShift,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    InstanceOf,
    In,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
}

impl From<BinaryOperator> for &'static str {
    fn from(operator: BinaryOperator) -> Self {
        use BinaryOperator::*;

        match operator {
            Equality => "==",
            InEquality => "!=",
            Identity => "===",
            NonIdentity => "!==",
            LessThan => "<",
            LessThanEqual => "<=",
            GreaterThan => ">",
            GreaterThanEqual => ">=",
            RightShift => ">>",
            LeftShift => "<<",
            UnsignedRightShift => ">>>",
            Addition => "+",
            Subtraction => "-",
            Multiplication => "*",
            Division => "/",
            Remainder => "%",
            InstanceOf => "instanceof",
            In => "in",
            BitwiseOr => "|",
            BitwiseXor => "^",
            BitwiseAnd => "&",
        }
    }
}

impl TryFrom<Opcode> for BinaryOperator {
    type Error = ParserError;

    fn try_from(opcode: Opcode) -> Result<Self, Self::Error> {
        match opcode {
            Opcode::Eq => Ok(Self::Equality),
            Opcode::StrictEq => Ok(Self::Identity),
            Opcode::Neq => Ok(Self::InEquality),
            Opcode::StrictNeq => Ok(Self::NonIdentity),
            Opcode::Less => Ok(Self::LessThan),
            Opcode::LessEq => Ok(Self::LessThanEqual),
            Opcode::Greater => Ok(Self::GreaterThan),
            Opcode::GreaterEq => Ok(Self::GreaterThanEqual),
            Opcode::Add | Opcode::AddN => Ok(Self::Addition),
            Opcode::Mul | Opcode::MulN => Ok(Self::Multiplication),
            Opcode::Div | Opcode::DivN => Ok(Self::Division),
            Opcode::Mod => Ok(Self::Remainder),
            Opcode::Sub | Opcode::SubN => Ok(Self::Subtraction),
            Opcode::LShift => Ok(Self::LeftShift),
            Opcode::RShift => Ok(Self::RightShift),
            Opcode::BitAnd => Ok(Self::BitwiseAnd),
            Opcode::BitXor => Ok(Self::BitwiseXor),
            Opcode::InstanceOf => Ok(Self::InstanceOf),
            Opcode::IsIn => Ok(Self::In),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not a valid Binary operation", opcode),
            )),
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    operator: BinaryOperator,
    operands: (Register, Register),
}

impl OpcodeStatement for BinaryExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (remaining, result) = tuple((le_u8, le_u8, le_u8))(input)?;
        let (register_byte, operand_1, operand_2) = result;

        let expression = Expression::Binary(Self {
            operator: BinaryOperator::try_from(opcode)?,
            operands: (Register::from(operand_1), Register::from(operand_2)),
        });

        let statement = Statement::Expression {
            register: Register::from(register_byte),
            expression,
        };

        Ok((remaining, statement))
    }
}

#[derive(Debug)]
pub enum AssignmentOperator {
    Assignment,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    LeftShift,
    RightShift,
    UnsignedShift,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
}

impl From<AssignmentOperator> for &'static str {
    fn from(operator: AssignmentOperator) -> Self {
        use AssignmentOperator::*;

        match operator {
            Assignment => "=",
            Addition => "+=",
            Subtraction => "-=",
            Multiplication => "*=",
            Division => "/=",
            Remainder => "%=",
            LeftShift => ">>=",
            RightShift => "<<=",
            UnsignedShift => ">>>=",
            BitwiseOr => "|=",
            BitwiseXor => "^=",
            BitwiseAnd => "&=",
        }
    }
}

#[derive(Debug)]
pub struct AssignmentExpression {
    operator: AssignmentOperator,
    right: Register,
}
