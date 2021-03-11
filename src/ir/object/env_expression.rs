use nom::{
    combinator::map,
    number::complete::{le_u16, le_u8},
};

use crate::{
    ir::{Expression, Register, Statement},
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

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

impl EnvExpression {
    fn parse_store(input: &[u8], is_long: bool) -> ParserResult<Self> {
        let (input, index) = match is_long {
            true => map(le_u16, EnvIndex::Word)(input),
            false => map(le_u8, EnvIndex::Byte)(input),
        }?;
        let (input, value) = map(le_u8, |x| Some(Register::Byte(x)))(input)?;

        Ok((input, Self { index, value }))
    }

    fn parse_load(input: &[u8], is_long: bool) -> ParserResult<Self> {
        let (input, index) = match is_long {
            true => map(le_u16, EnvIndex::Word)(input),
            false => map(le_u8, EnvIndex::Byte)(input),
        }?;
        Ok((input, Self { index, value: None }))
    }
}

impl OpcodeStatement for EnvExpression {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, register) = map(le_u8, Register::Byte)(input)?;

        let (input, env_expression) = match opcode {
            Opcode::StoreToEnvironment | Opcode::StoreNPToEnvironment => {
                Self::parse_store(input, false)
            }
            Opcode::StoreToEnvironmentL | Opcode::StoreNPToEnvironmentL => {
                Self::parse_store(input, true)
            }
            Opcode::LoadFromEnvironment => Self::parse_load(input, false),
            Opcode::LoadFromEnvironmentL => Self::parse_load(input, true),
            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not an EnvExpression", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register,
            expression: Expression::EnvExp(env_expression),
        };
        Ok((input, statement))
    }
}
