use std::{cell::RefCell, rc::Rc};

use super::{Expression, Identifier, MemberExpression};

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

#[derive(Debug)]
pub struct BinaryExpression<'a> {
    operator: BinaryOperator,
    left: Rc<RefCell<Expression<'a>>>,
    right: Rc<RefCell<Expression<'a>>>,
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
pub enum Assignee<'a> {
    Identifier(Identifier),
    Expression(Rc<RefCell<MemberExpression<'a>>>),
}

#[derive(Debug)]
pub struct AssignmentExpression<'a> {
    operator: AssignmentOperator,
    left: Assignee<'a>,
    right: Rc<RefCell<Expression<'a>>>,
}
