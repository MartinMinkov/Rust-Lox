use super::{ExpressionNode, Token};

#[derive(Debug, Clone)]
pub enum Statement {
    PrintStatement(Box<ExpressionNode>),
    IfStatement(Box<ExpressionNode>, Box<Statement>, Option<Box<Statement>>),
    WhileStatement(Box<ExpressionNode>, Box<Statement>),
    ExpressionStatement(Box<ExpressionNode>),
    VariableDeclaration(Token, Option<Box<ExpressionNode>>),
    FunctionDeclaration(Token, Vec<Token>, Vec<Statement>),
    BlockStatement(Vec<Statement>),
}
