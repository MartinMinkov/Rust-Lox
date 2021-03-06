use super::Literal;
use super::{BinaryOperator, LogicalOperator, Statement, TernaryOperator, Token, UnaryOperator};
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    GetExpression(Identifier, Box<ExpressionNode>),
    SetExpression(Box<ExpressionNode>, Identifier, Box<ExpressionNode>),
    FunctionExpression(FunctionExpression),
    Literal(Literal),
    Unary(UnaryOperator, Box<ExpressionNode>),
    Variable(Variable),
    Assignment(Variable, Box<ExpressionNode>),
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
            Expression::GetExpression(name, expr) => {
                write!(f, "(get {} {})", name.get_name(), expr)
            }
            Expression::SetExpression(object, name, value) => {
                write!(f, "(set {} {} {})", object, name.get_name(), value)
            }
            Expression::Literal(val) => write!(f, "{}", val),
            Expression::Unary(operator, right) => write!(f, "({} {})", operator, right),
            Expression::Variable(variable) => write!(f, "{}", variable.get_identifier().get_name()),
            Expression::Assignment(variable, expr) => {
                write!(f, "{} {}", variable.get_identifier().get_name(), expr)
            }
            Expression::And(left_expr, op, right_expr)
            | Expression::Or(left_expr, op, right_expr) => {
                write!(f, "{} {} {}", left_expr, op, right_expr)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionNode {
    line: usize,
    expr: Expression,
}

impl ExpressionNode {
    pub fn new(line: usize, expr: Expression) -> Self {
        Self { line, expr }
    }

    pub fn expr(&self) -> &Expression {
        &self.expr
    }

    pub fn expr_mut(&mut self) -> &mut Expression {
        &mut self.expr
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    name: String,
    line: usize,
}

impl Identifier {
    pub fn token_to_id(token: Token) -> Self {
        Self {
            name: token.lexeme,
            line: token.line,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    identifier: Identifier,
    depth: Option<usize>,
}

impl Variable {
    pub fn default(identifier: Identifier) -> Self {
        return Self {
            identifier,
            depth: None,
        };
    }

    pub fn get_identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn get_depth(&self) -> Option<usize> {
        self.depth
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = Some(depth)
    }
}

#[derive(Clone, Debug)]
pub enum Function {
    Declaration(FunctionDeclaration),
    Expression(FunctionExpression),
}

pub trait FunctionInfo {
    fn identifier(&self) -> String;
    fn parameters(&self) -> &Vec<Identifier>;
    fn body(&self) -> &Vec<Statement>;
}

impl FunctionInfo for Function {
    fn identifier(&self) -> String {
        match &self {
            Function::Declaration(func) => func.identifier.get_name(),
            Function::Expression(_) => "fn anonymous".to_string(),
        }
    }

    fn parameters(&self) -> &Vec<Identifier> {
        match &self {
            Function::Declaration(func) => &func.parameters,
            Function::Expression(func) => &func.parameters,
        }
    }

    fn body(&self) -> &Vec<Statement> {
        match &self {
            Function::Declaration(func) => &func.body,
            Function::Expression(func) => &func.body,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub identifier: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: Vec<Statement>,
}

impl FunctionDeclaration {
    pub fn new(identifier: Identifier, parameters: Vec<Identifier>, body: Vec<Statement>) -> Self {
        Self {
            identifier,
            parameters,
            body,
        }
    }
    pub fn get_identifier(&self) -> String {
        self.identifier.get_name()
    }

    pub fn get_parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    pub fn get_body(&self) -> &Vec<Statement> {
        &self.body
    }
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "FunctionDeclaration: {}", self.identifier.get_name())
    }
}

impl FunctionInfo for FunctionDeclaration {
    fn identifier(&self) -> String {
        self.get_identifier()
    }

    fn parameters(&self) -> &Vec<Identifier> {
        self.get_parameters()
    }

    fn body(&self) -> &Vec<Statement> {
        self.get_body()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    pub parameters: Vec<Identifier>,
    pub body: Vec<Statement>,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<Identifier>, body: Vec<Statement>) -> Self {
        Self { parameters, body }
    }

    pub fn get_identifier(&self) -> String {
        "fn anonymous".to_string()
    }

    pub fn get_parameters(&self) -> &Vec<Identifier> {
        &self.parameters
    }

    pub fn get_body(&self) -> &Vec<Statement> {
        &self.body
    }
}

impl Display for FunctionExpression {
    fn fmt(&self, _: &mut Formatter) -> FmtResult {
        println!("FunctionExpression");
        for param in &self.parameters {
            println!("{}", param.get_name());
        }
        Ok(())
    }
}

impl FunctionInfo for FunctionExpression {
    fn identifier(&self) -> String {
        self.get_identifier()
    }

    fn parameters(&self) -> &Vec<Identifier> {
        self.get_parameters()
    }

    fn body(&self) -> &Vec<Statement> {
        self.get_body()
    }
}
