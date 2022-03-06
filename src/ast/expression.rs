use super::Literal;
use super::{BinaryOperator, LogicalOperator, Statement, TernaryOperator, Token, UnaryOperator};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct ExpressionNode {
    line: u16,
    expr: Expression,
}

impl ExpressionNode {
    pub fn new(line: u16, expr: Expression) -> Self {
        Self { line, expr }
    }

    pub fn expression(&self) -> &Expression {
        &self.expr
    }

    pub fn line(&self) -> u16 {
        self.line
    }
}

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub identifier: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>,
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "FunctionDeclaration: {}", self.identifier)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>,
}

impl Display for FunctionExpression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "FunctionExpression");
        for param in &self.parameters {
            write!(f, "{}", param);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Function {
    Declaration(FunctionDeclaration),
    Expression(FunctionExpression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    TernaryExpression(
        Box<ExpressionNode>,
        TernaryOperator,
        Box<ExpressionNode>,
        Box<ExpressionNode>,
    ),
    BinaryExpression(Box<ExpressionNode>, BinaryOperator, Box<ExpressionNode>),
    Grouping(Box<ExpressionNode>),
    CallExpression(Box<ExpressionNode>, Token, Vec<ExpressionNode>),
    FunctionExpression(FunctionExpression),
    Literal(Literal),
    Unary(UnaryOperator, Box<ExpressionNode>),
    Variable(Token),
    Assignment(Token, Box<ExpressionNode>),
    Or(Box<ExpressionNode>, LogicalOperator, Box<ExpressionNode>),
    And(Box<ExpressionNode>, LogicalOperator, Box<ExpressionNode>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &*self {
            Expression::TernaryExpression(if_expr, operator, left, right) => {
                write!(f, "({} {} ({} {}))", if_expr, operator, left, right)
            }
            Expression::BinaryExpression(left, operator, right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expression::Grouping(expr) => write!(f, "(group {})", expr),
            Expression::CallExpression(callee_expr, _paren, _args) => {
                write!(f, "(call {})", callee_expr)
            }
            Expression::FunctionExpression(expr) => {
                write!(f, "({})", expr)
            }
            Expression::Literal(val) => write!(f, "{}", val),
            Expression::Unary(operator, right) => write!(f, "({} {})", operator, right),
            Expression::Variable(var) => write!(f, "{}", var.lexeme),
            Expression::Assignment(var, expr) => write!(f, "{} {}", var.lexeme, expr),
            Expression::And(left_expr, op, right_expr)
            | Expression::Or(left_expr, op, right_expr) => {
                write!(f, "{} {} {}", left_expr, op, right_expr)
            }
        }
    }
}
