use std::{cell::RefCell, rc::Rc};

use super::{Expression, Literal};

#[derive(Debug)]
pub struct ThisExpression;

#[derive(Debug)]
pub struct ArrayExpression<'a> {
    elements: Vec<Option<Rc<RefCell<Expression<'a>>>>>,
}

#[derive(Debug)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug)]
struct Property<'a> {
    key: Literal,
    value: Rc<RefCell<Expression<'a>>>,
    kind: PropertyKind,
}

#[derive(Debug)]
pub struct ObjectExpression<'a> {
    properties: Vec<Property<'a>>,
}
