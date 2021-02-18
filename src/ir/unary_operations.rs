use std::{cell::RefCell, rc::Rc};

use super::Expression;

#[derive(Debug)]
pub enum UnaryOperator {
    Negation,
    Plus,
    LogicalNot,
    BitwiseNot,
    TypeOf,
    Void,
    Delete,
}

impl From<UnaryOperator> for &'static str {
    fn from(operator: UnaryOperator) -> Self {
        use UnaryOperator::*;

        match operator {
            Negation => "-",
            Plus => "+",
            LogicalNot => "!",
            BitwiseNot => "~",
            TypeOf => "typeof",
            Void => "void",
            Delete => "delete",
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression<'a> {
    operator: UnaryOperator,
    prefix: bool,
    argument: Rc<RefCell<Expression<'a>>>,
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
pub struct UpdateExpression<'a> {
    operator: UpdateOperator,
    prefix: bool,
    argument: Rc<RefCell<Expression<'a>>>,
}
