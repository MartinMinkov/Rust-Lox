use super::{ExpressionNode, FunctionDeclaration, Identifier};

#[derive(Debug, Clone)]
pub enum Statement {
    PrintStatement(Box<ExpressionNode>),
    IfStatement(Box<ExpressionNode>, Box<Statement>, Option<Box<Statement>>),
    WhileStatement(Box<ExpressionNode>, Box<Statement>),
    ExpressionStatement(Box<ExpressionNode>),
    VariableDeclaration(Identifier, Option<Box<ExpressionNode>>),
    FunctionDeclaration(FunctionDeclaration),
    BlockStatement(Vec<Statement>),
    ReturnStatement(Option<Box<ExpressionNode>>),
}
