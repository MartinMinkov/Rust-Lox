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
    parameters: Vec<Token>,
    body: Vec<Statement>,
}

impl FunctionDeclaration {
    pub fn new(identifier: Token, parameters: Vec<Token>, body: Vec<Statement>) -> Self {
        Self {
            identifier,
            parameters,
            body,
        }
    }
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "FunctionDeclaration: {}", self.identifier)
    }
}

impl FunctionInfo for FunctionDeclaration {
    fn identifier(&self) -> &str {
        self.identifier()
    }

    fn parameters(&self) -> Vec<Token> {
        self.parameters()
    }

    fn body(&self) -> Vec<Statement> {
        self.body()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    parameters: Vec<Token>,
    body: Vec<Statement>,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<Token>, body: Vec<Statement>) -> Self {
        Self { parameters, body }
    }
}

impl Display for FunctionExpression {
    fn fmt(&self, _: &mut Formatter) -> FmtResult {
        println!("FunctionExpression");
        for param in &self.parameters {
            println!("{}", param);
        }
        Ok(())
    }
}

impl FunctionInfo for FunctionExpression {
    fn identifier(&self) -> &str {
        self.identifier()
    }

    fn parameters(&self) -> Vec<Token> {
        self.parameters()
    }

    fn body(&self) -> Vec<Statement> {
        self.body()
    }
}

#[derive(Clone, Debug)]
pub enum Function {
    Declaration(FunctionDeclaration),
    Expression(FunctionExpression),
}

pub trait FunctionInfo {
    fn identifier(&self) -> &str;
    fn parameters(&self) -> Vec<Token>;
    fn body(&self) -> Vec<Statement>;
}

impl FunctionInfo for Function {
    fn identifier(&self) -> &str {
        match &self {
            Function::Declaration(func) => func.identifier.lexeme.as_str(),
            Function::Expression(func) => "fn anonymous",
        }
    }

    fn parameters(&self) -> Vec<Token> {
        match &self {
            Function::Declaration(func) => func.parameters.clone(),
            Function::Expression(func) => func.parameters.clone(),
        }
    }

    fn body(&self) -> Vec<Statement> {
        match &self {
            Function::Declaration(func) => func.body.clone(),
            Function::Expression(func) => func.body.clone(),
        }
    }
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
