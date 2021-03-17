use std::convert::TryFrom;

use nom::{
    combinator::map,
    number::complete::{le_u16, le_u32, le_u8},
};

use crate::{
    builtins::Builtins,
    ir::{Expression, Register, Statement},
    opcodes::Opcode,
    parsers::{OpcodeStatement, ParserError, ParserResult},
};

#[derive(Debug)]
pub enum FunctionIndex {
    Register(Register),
    Word(u16),
    Dword(u32),
}

#[derive(Debug)]
pub enum ArgsNo {
    Byte(u8),
    Dword(u32),
}

#[derive(Debug)]
pub enum FunctionType {
    Normal(FunctionIndex),
    Constructor(FunctionIndex),
    BuiltIn(Builtins),
}

#[derive(Debug)]
pub struct FrameCall {
    no_of_arguments: ArgsNo,
    function_type: FunctionType,
}

impl FrameCall {
    fn parse_call(is_constructor: bool, is_long: bool, input: &[u8]) -> ParserResult<Self> {
        let (input, function_register) = map(le_u8, Register::Byte)(input)?;
        let function = FunctionIndex::Register(function_register);

        let function_type = if is_constructor {
            FunctionType::Constructor(function)
        } else {
            FunctionType::Normal(function)
        };

        let (input, no_of_arguments) = match is_long {
            true => map(le_u32, ArgsNo::Dword)(input),
            false => map(le_u8, ArgsNo::Byte)(input),
        }?;

        let frame_call = Self {
            no_of_arguments,
            function_type,
        };
        Ok((input, frame_call))
    }

    fn parse_direct(is_long: bool, input: &[u8]) -> ParserResult<Self> {
        let (input, no_of_arguments) = map(le_u8, ArgsNo::Byte)(input)?;

        let (input, function) = match is_long {
            true => map(le_u32, FunctionIndex::Dword)(input),
            false => map(le_u16, FunctionIndex::Word)(input),
        }?;

        let frame_call = Self {
            no_of_arguments,
            function_type: FunctionType::Normal(function),
        };
        Ok((input, frame_call))
    }

    fn parse_builtin(input: &[u8]) -> ParserResult<Self> {
        let (input, builtin_no) = le_u8(input)?;
        let builtin = Builtins::try_from(builtin_no)?;

        let (input, no_of_arguments) = map(le_u8, ArgsNo::Byte)(input)?;

        let frame_call = Self {
            no_of_arguments,
            function_type: FunctionType::BuiltIn(builtin),
        };
        Ok((input, frame_call))
    }
}

impl OpcodeStatement for FrameCall {
    fn parse(opcode: Opcode, input: &[u8]) -> ParserResult<Statement> {
        let (input, register) = map(le_u8, Register::Byte)(input)?;

        let (input, frame_call) = match opcode {
            Opcode::Call => Self::parse_call(false, false, input),
            Opcode::Construct => Self::parse_call(true, false, input),
            Opcode::CallLong => Self::parse_call(false, true, input),
            Opcode::ConstructLong => Self::parse_call(true, true, input),

            Opcode::CallDirect => Self::parse_direct(false, input),
            Opcode::CallDirectLongIndex => Self::parse_direct(true, input),

            Opcode::CallBuiltin => Self::parse_builtin(input),

            _ => Err(ParserError::new(
                "Opcode",
                format!("{:?} is not an FrameCall opcode", opcode),
            ))?,
        }?;

        let statement = Statement::Expression {
            register,
            expression: Expression::FrameCall(frame_call),
        };
        Ok((input, statement))
    }
}

#[derive(Debug)]
pub struct CallExpression {
    function: FunctionIndex,
    arguments: Vec<Register>,
}
