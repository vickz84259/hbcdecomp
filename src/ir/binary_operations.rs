use super::Register;

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
pub struct BinaryExpression {
    operator: BinaryOperator,
    operands: (Register, Register),
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
