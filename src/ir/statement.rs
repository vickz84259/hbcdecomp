use super::Expression;

#[derive(Debug)]
pub enum Statement<'a> {
    Declaration,
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
    Nop,
}
