use super::Literal;
use super::{Token, TokenType};
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

pub trait OperatorTokenType {
	fn token_type(&self) -> TokenType;
}

#[derive(Debug, Clone)]
pub enum Statement {
	PrintStatement(Box<ExpressionNode>),
	IfStatement(Box<ExpressionNode>, Box<Statement>, Option<Box<Statement>>),
	ExpressionStatement(Box<ExpressionNode>),
	VariableDeclaration(Token, Option<Box<ExpressionNode>>),
	BlockStatement(Vec<Statement>),
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
	Literal(Literal),
	Unary(UnaryOperator, Box<ExpressionNode>),
	Variable(Token),
	Assignment(Token, Box<ExpressionNode>),
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
			Expression::Literal(val) => write!(f, "{}", val),
			Expression::Unary(operator, right) => write!(f, "({} {})", operator, right),
			Expression::Variable(var) => write!(f, "{}", var.lexeme),
			Expression::Assignment(var, expr) => write!(f, "{} {}", var.lexeme, expr),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOperator {
	MINUS,
	BANG,
}

impl OperatorTokenType for UnaryOperator {
	fn token_type(&self) -> TokenType {
		match *self {
			UnaryOperator::MINUS => TokenType::MINUS,
			UnaryOperator::BANG => TokenType::BANG,
		}
	}
}

impl Display for UnaryOperator {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match self {
			UnaryOperator::MINUS => write!(f, "-"),
			UnaryOperator::BANG => write!(f, "!"),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
	MINUS,
	PLUS,
	BANGEQUAL,
	EQUALEQUAL,
	GREATER,
	GREATEREQUAL,
	LESS,
	LESSEQUAL,
	SLASH,
	STAR,
	COMMA,
}

impl OperatorTokenType for BinaryOperator {
	fn token_type(&self) -> TokenType {
		match *self {
			BinaryOperator::MINUS => TokenType::MINUS,
			BinaryOperator::PLUS => TokenType::PLUS,
			BinaryOperator::BANGEQUAL => TokenType::BANGEQUAL,
			BinaryOperator::EQUALEQUAL => TokenType::EQUALEQUAL,
			BinaryOperator::GREATER => TokenType::GREATER,
			BinaryOperator::GREATEREQUAL => TokenType::GREATEREQUAL,
			BinaryOperator::LESS => TokenType::LESS,
			BinaryOperator::LESSEQUAL => TokenType::LESSEQUAL,
			BinaryOperator::SLASH => TokenType::SLASH,
			BinaryOperator::STAR => TokenType::STAR,
			BinaryOperator::COMMA => TokenType::COMMA,
		}
	}
}

impl Display for BinaryOperator {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match self {
			BinaryOperator::MINUS => write!(f, "-"),
			BinaryOperator::PLUS => write!(f, "+"),
			BinaryOperator::BANGEQUAL => write!(f, "!="),
			BinaryOperator::EQUALEQUAL => write!(f, "=="),
			BinaryOperator::GREATER => write!(f, ">"),
			BinaryOperator::GREATEREQUAL => write!(f, ">="),
			BinaryOperator::LESS => write!(f, "<"),
			BinaryOperator::LESSEQUAL => write!(f, "<="),
			BinaryOperator::SLASH => write!(f, "/"),
			BinaryOperator::STAR => write!(f, "*"),
			BinaryOperator::COMMA => write!(f, ","),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum TernaryOperator {
	QUESTIONMARK,
}

impl OperatorTokenType for TernaryOperator {
	fn token_type(&self) -> TokenType {
		match *self {
			TernaryOperator::QUESTIONMARK => TokenType::QUESTIONMARK,
		}
	}
}

impl Display for TernaryOperator {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match self {
			TernaryOperator::QUESTIONMARK => write!(f, "?"),
		}
	}
}
