use super::{Token, TokenType};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Literal {
	STRINGLITERAL(String),
	NUMBERLITERAL(f64),
}

impl Display for Literal {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match &*self {
			Literal::STRINGLITERAL(val) => write!(f, "{}", val),
			Literal::NUMBERLITERAL(val) => write!(f, "{}", val),
		}
	}
}

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
				write!(f, "BINARY EXPRESSION: {} {} {}", left, operator, right)
			}
			Expression::Grouping(expr) => write!(f, "GROUPING EXPRESSION: {}", expr),
			Expression::Literal(val) => write!(f, "LITERAL EXPRESSION {}", val),
			Expression::Unary(token, right) => write!(f, "Unary EXPRESSION: {} {}", token.lexeme, right),
		}
	}
}

impl Expression {
	pub fn test_expr() {
		let unary_expr = Expression::Unary(
			Token::new(TokenType::MINUS, "-".to_string(), None, 1),
			Box::new(Expression::Literal(Literal::NUMBERLITERAL(123.0))),
		);
		println!("Printing AST {}", unary_expr);
	}
}
