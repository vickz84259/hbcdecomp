mod binary_operations;
mod declaration;
mod expression;
mod identifier;
mod literal;
mod object;
mod unary_operations;

pub use binary_operations::*;
pub use declaration::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use object::*;
pub use unary_operations::*;

use super::bytecode_file_format::{BytecodeFile, FunctionHeader};

#[derive(Debug)]
pub enum Statement<'a> {
    Declaration,
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
    Nop,
}

#[derive(Debug)]
pub struct Function<'a> {
    header: &'a FunctionHeader,
    body: Vec<Statement<'a>>,
}

struct Program<'a> {
    bytecode: BytecodeFile<'a>,
    functions: Vec<Function<'a>>,
}
