use std::{cell::RefCell, rc::Rc};

use super::{
    ArrayExpression, AssignmentExpression, BinaryExpression, Function, Identifier, Literal,
    ObjectExpression, ThisExpression, UnaryExpression, UpdateExpression,
};

#[derive(Debug)]
pub struct MemberExpression<'a> {
    object: Identifier,
    property: Rc<RefCell<Expression<'a>>>,
    computed: bool,
}

#[derive(Debug)]
pub struct CallExpression<'a> {
    callee: Identifier,
    arguments: Vec<Rc<RefCell<Expression<'a>>>>,
}

#[derive(Debug)]
pub struct NewExpression<'a> {
    callee: Identifier,
    arguments: Vec<Rc<RefCell<Expression<'a>>>>,
}

#[derive(Debug)]
pub struct FunctionExpression<'a> {
    id: Option<Identifier>,
    value: Option<Function<'a>>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Literal(Literal),
    Member(MemberExpression<'a>),
    Call(CallExpression<'a>),
    New(NewExpression<'a>),
    Assignment(AssignmentExpression<'a>),
    Binary(BinaryExpression<'a>),
    Unary(UnaryExpression<'a>),
    Update(UpdateExpression<'a>),
    Function(FunctionExpression<'a>),
    This(ThisExpression),
    Array(ArrayExpression<'a>),
    Object(ObjectExpression<'a>),
}
