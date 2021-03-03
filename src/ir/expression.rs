use super::{
    BinaryExpression, CallExpression, EnvExpression, FrameCall, Literal, NewArrayExpression,
    NewObjectExpression, Object, ObjectExpression, Register, UnaryExpression,
};

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Register(Register),
    NewObject(NewObjectExpression),
    NewArray(NewArrayExpression),
    EnvExp(EnvExpression),
    Object(Object),
    ObjExp(ObjectExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    FrameCall(FrameCall),
    CallExp(CallExpression),
}
