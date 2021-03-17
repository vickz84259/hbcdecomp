use nom::{combinator::map, number::complete::le_u8};

use crate::parsers::ParserResult;

use super::{
    BinaryExpression, CallExpression, EnvExpression, FrameCall, Literal, NewArrayExpression,
    NewObjectExpression, Object, ObjectExpression, Register, Statement, UnaryExpression,
};

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Register(Register),
    NewObject(NewObjectExpression),
    NewArray(NewArrayExpression),
    LoadFromEnv(EnvExpression),
    Object(Object),
    ObjExp(ObjectExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    FrameCall(FrameCall),
    CallExp(CallExpression),
    NewTarget,
}

impl Expression {
    pub fn parse_new_target(input: &[u8]) -> ParserResult<Statement> {
        map(le_u8, |byte| Statement::Expression {
            register: Register::Byte(byte),
            expression: Self::NewTarget,
        })(input)
    }
}
