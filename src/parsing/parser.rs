use super::Error;
use super::{
	BinaryOperator, Expression, ExpressionNode, OperatorTokenType, TernaryOperator, UnaryOperator,
};
use super::{Literal, Token, TokenType};

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
	MissingExpr(Token),
}

impl ParseError {
	fn print(&self) {
		let (token, message) = match self {
			ParseError::MissingExpr(token) => (token, String::from("Expect expression")),
		};
		eprintln!(
			"[line {}] Error at {}: {}.",
			token.line, token.lexeme, message
		);
	}
}

pub struct Parser {
	pub tokens: Vec<Token>,
	current_token: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			current_token: 0,
		}
	}
	pub fn parse(&mut self) -> Result<ExpressionNode, ()> {
		match self.expression() {
			Ok(expr) => return Ok(expr),
			Err(e) => {
				e.print();
				Err(())
			}
		}
	}

	fn expression(&mut self) -> ParseResult<ExpressionNode> {
		self.comma()
	}

	fn comma(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.ternary()?;

		if let Some(binary_op) = self.match_operator_type(vec![BinaryOperator::COMMA]) {
			let right_expr = self.expression()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn ternary(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.equality()?;

		if let Some(ternary_op) = self.match_operator_type(vec![TernaryOperator::QUESTIONMARK]) {
			let left_expr = self.expression()?;

			self.consume(
				TokenType::COLON,
				String::from("Expect ':' after then branch of ternary expression."),
			);

			let right_expr = self.expression()?;
			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::TernaryExpression(
					Box::new(expr),
					ternary_op,
					Box::new(left_expr),
					Box::new(right_expr),
				),
			));
		}
		Ok(expr)
	}

	fn equality(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.comparison()?;

		if let Some(binary_op) =
			self.match_operator_type(vec![BinaryOperator::BANGEQUAL, BinaryOperator::EQUALEQUAL])
		{
			let right_expr = self.comparison()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn comparison(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.term()?;

		if let Some(binary_op) = self.match_operator_type(vec![
			BinaryOperator::GREATER,
			BinaryOperator::GREATEREQUAL,
			BinaryOperator::LESS,
			BinaryOperator::LESSEQUAL,
		]) {
			let right_expr = self.term()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn term(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.factor()?;

		if let Some(binary_op) =
			self.match_operator_type(vec![BinaryOperator::MINUS, BinaryOperator::PLUS])
		{
			let right_expr = self.factor()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn factor(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.unary()?;

		if let Some(binary_op) =
			self.match_operator_type(vec![BinaryOperator::SLASH, BinaryOperator::STAR])
		{
			let right_expr = self.unary()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn unary(&mut self) -> ParseResult<ExpressionNode> {
		if let Some(unary_op) =
			self.match_operator_type(vec![UnaryOperator::BANG, UnaryOperator::MINUS])
		{
			let right_expr = self.unary()?;
			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::Unary(unary_op, Box::new(right_expr)),
			));
		}
		return self.primary();
	}

	fn primary(&mut self) -> ParseResult<ExpressionNode> {
		let current_token = self.advance();
		let current_line = self.current_line();

		match current_token.typ {
			TokenType::TRUE => {
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(Literal::BOOLEAN(true)),
				));
			}
			TokenType::FALSE => {
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(Literal::BOOLEAN(false)),
				));
			}
			TokenType::NIL => {
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(Literal::NIL),
				));
			}
			TokenType::NUMBER | TokenType::STRING => {
				let token = self.previous();
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(token.literal.unwrap()),
				));
			}
			TokenType::LEFTPAREN => {
				let expr = self.expression()?;
				self.consume(
					TokenType::RIGHTPAREN,
					String::from("Expect ')' after expression."),
				);
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Grouping(Box::new(expr)),
				));
			}
			_ => return Err(ParseError::MissingExpr(current_token)),
		};
	}

	fn match_operator_type<T: OperatorTokenType>(&mut self, types: Vec<T>) -> Option<T> {
		for typ in types {
			if self.peek().typ == OperatorTokenType::token_type(&typ) {
				self.advance();
				return Some(typ);
			}
		}
		None
	}

	fn check_token_type(&self, typ: TokenType) -> bool {
		if self.is_at_end() {
			false;
		}
		self.peek().typ == typ
	}

	fn peek(&self) -> Token {
		self.tokens.get(self.current_token).unwrap().clone()
	}

	fn is_at_end(&self) -> bool {
		self.peek().typ == TokenType::EOF
	}

	fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.current_token = self.current_token + 1;
		}
		self.previous()
	}

	fn current_line(&mut self) -> u16 {
		self.peek().line
	}

	fn previous(&self) -> Token {
		if self.current_token == 0 {
			return self.tokens.get(self.current_token).unwrap().clone();
		} else {
			self.tokens.get(self.current_token - 1).unwrap().clone()
		}
	}

	fn consume(&mut self, typ: TokenType, message: String) -> Option<Token> {
		if self.check_token_type(typ) {
			return Some(self.advance());
		}
		self.error(self.peek(), message);
		None
	}

	fn error(&self, token: Token, message: String) {
		Error::error(token.line, message);
	}

	fn synchronize(&mut self) {
		self.advance();
		while !self.is_at_end() {
			if self.previous().typ == TokenType::SEMICOLON {
				break;
			}
			match self.peek().typ {
				TokenType::CLASS
				| TokenType::FUN
				| TokenType::VAR
				| TokenType::FOR
				| TokenType::IF
				| TokenType::WHILE
				| TokenType::PRINT
				| TokenType::RETURN => break,
				_ => {}
			}
			self.advance();
		}
	}
}
