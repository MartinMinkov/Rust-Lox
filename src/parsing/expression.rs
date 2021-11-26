use super::Literal;
use super::Token;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Expression {
	BinaryExpression(Box<Expression>, Token, Box<Expression>),
	Grouping(Box<Expression>),
	Literal(Literal),
	Unary(Token, Box<Expression>),
}

impl Display for Expression {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match &*self {
			Expression::BinaryExpression(left, operator, right) => {
				write!(f, "({} {} {})", operator, left, right)
			}
			Expression::Grouping(expr) => write!(f, "(group {})", expr),
			Expression::Literal(val) => write!(f, "{}", val),
			Expression::Unary(token, right) => write!(f, "({} {})", token.lexeme, right),
		}
	}
}
