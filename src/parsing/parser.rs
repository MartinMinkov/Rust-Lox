use super::Error;
use super::Expression;
use super::{Literal, Token, TokenType};

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
	pub fn parse(&mut self) -> Option<Expression> {
		self.expression()
	}

	fn expression(&mut self) -> Option<Expression> {
		self.comma()
	}

	fn comma(&mut self) -> Option<Expression> {
		let mut expr = self.ternary();
		while self.match_token_types(vec![TokenType::COMMA]) {
			let op = self.previous();
			let right_expr = self.expression();
			expr =
				right_expr.map(|r| Expression::BinaryExpression(Box::new(expr.unwrap()), op, Box::new(r)));
		}
		expr
	}

	fn ternary(&mut self) -> Option<Expression> {
		let mut expr = self.equality();
		while self.match_token_types(vec![TokenType::QUESTIONMARK]) {
			let op = self.previous();
			let left_expr = self.expression();
			self.consume(
				TokenType::COLON,
				String::from("Expect ':' after then branch of ternary expression."),
			);
			let right_expr = self.expression();
			expr = match (expr, left_expr, right_expr) {
				(Some(expr), Some(left_expr), Some(right_expr)) => Some(Expression::TernaryExpression(
					Box::new(expr),
					op,
					Box::new(left_expr),
					Box::new(right_expr),
				)),
				_ => None,
			}
		}
		expr
	}

	fn equality(&mut self) -> Option<Expression> {
		let mut expr = self.comparison();
		while self.match_token_types(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
			let op = self.previous();
			let right_expr = self.comparison();
			expr =
				right_expr.map(|r| Expression::BinaryExpression(Box::new(expr.unwrap()), op, Box::new(r)));
		}
		expr
	}

	fn comparison(&mut self) -> Option<Expression> {
		let mut expr = self.term();

		while self.match_token_types(vec![
			TokenType::GREATER,
			TokenType::GREATEREQUAL,
			TokenType::LESSEQUAL,
		]) {
			let op = self.previous();
			let right_expr = self.term();
			expr =
				right_expr.map(|r| Expression::BinaryExpression(Box::new(expr.unwrap()), op, Box::new(r)));
		}
		expr
	}

	fn term(&mut self) -> Option<Expression> {
		let mut expr = self.factor();
		while self.match_token_types(vec![TokenType::MINUS, TokenType::PLUS]) {
			let op = self.previous();
			let right_expr = self.factor();
			expr =
				right_expr.map(|r| Expression::BinaryExpression(Box::new(expr.unwrap()), op, Box::new(r)));
		}
		expr
	}

	fn factor(&mut self) -> Option<Expression> {
		let mut expr = self.unary();
		while self.match_token_types(vec![TokenType::SLASH, TokenType::STAR]) {
			let op = self.previous();
			let right_expr = self.unary();
			expr =
				right_expr.map(|r| Expression::BinaryExpression(Box::new(expr.unwrap()), op, Box::new(r)));
		}
		expr
	}

	fn unary(&mut self) -> Option<Expression> {
		if self.match_token_types(vec![TokenType::BANG, TokenType::MINUS]) {
			let op = self.previous();
			let right_expr = self.unary();
			match right_expr {
				Some(right_expr) => return Some(Expression::Unary(op, Box::new(right_expr))),
				None => return None,
			}
		}
		return self.primary();
	}

	fn primary(&mut self) -> Option<Expression> {
		if self.match_token_types(vec![TokenType::TRUE]) {
			return Some(Expression::Literal(Literal::BOOLEAN(true)));
		} else if self.match_token_types(vec![TokenType::FALSE]) {
			return Some(Expression::Literal(Literal::BOOLEAN(false)));
		} else if self.match_token_types(vec![TokenType::NIL]) {
			return Some(Expression::Literal(Literal::NIL));
		}

		if self.match_token_types(vec![TokenType::NUMBER, TokenType::STRING]) {
			let token = self.previous();
			return Some(Expression::Literal(token.literal.unwrap()));
		}

		if self.match_token_types(vec![TokenType::LEFTPAREN]) {
			let expr = self.expression();
			self.consume(
				TokenType::RIGHTPAREN,
				String::from("Expect ')' after expression."),
			);
			return Some(Expression::Grouping(Box::new(expr.unwrap())));
		}
		Error::report_parse_error(self.peek(), String::from("Expect expression."));
		None
	}

	fn match_token_types(&mut self, types: Vec<TokenType>) -> bool {
		for typ in types {
			if self.check(typ) {
				self.advance();
				return true;
			}
		}
		false
	}

	fn check(&self, typ: TokenType) -> bool {
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

	fn previous(&self) -> Token {
		self.tokens.get(self.current_token - 1).unwrap().clone()
	}

	fn consume(&mut self, typ: TokenType, message: String) -> Option<Token> {
		if self.check(typ) {
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
