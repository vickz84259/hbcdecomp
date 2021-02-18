use super::{Expression, Function, Identifier};

#[derive(Debug)]
struct FunctionDeclaration<'a> {
    id: Option<Identifier>,
    value: Option<Function<'a>>,
}

#[derive(Debug)]
struct VariableDeclaration<'a> {
    id: Option<Identifier>,
    value: Option<Expression<'a>>,
}

enum Declaration<'a> {
    Function(FunctionDeclaration<'a>),
    Variable(VariableDeclaration<'a>),
}
