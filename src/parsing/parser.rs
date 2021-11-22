use super::Expression;
use super::{Literal, Token, TokenType};

pub struct Parser {
	pub tokens: Vec<Token>,
	current_token: usize,
}

impl Parser {
	pub fn new(&self, tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			current_token: 0,
		}
	}

	pub fn expression(&mut self) -> Expression {
		self.equality()
	}

	pub fn equality(&mut self) -> Expression {
		let mut expr = self.comparison();

		while self.match_token_types(vec![TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
			let op = self.previous();
			let right_expr = self.comparison();
			expr = Expression::BinaryExpression(Box::new(expr), op, Box::new(right_expr));
		}
		expr
	}

	pub fn comparison(&mut self) -> Expression {
		let mut expr = self.term();

		while self.match_token_types(vec![
			TokenType::GREATER,
			TokenType::GREATEREQUAL,
			TokenType::LESSEQUAL,
		]) {
			let op = self.previous();
			let right_expr = self.term();
			expr = Expression::BinaryExpression(Box::new(expr), op, Box::new(right_expr));
		}
		expr
	}

	pub fn term(&mut self) -> Expression {
		let mut expr = self.factor();
		while self.match_token_types(vec![TokenType::MINUS, TokenType::PLUS]) {
			let op = self.previous();
			let right_expr = self.factor();
			expr = Expression::BinaryExpression(Box::new(expr), op, Box::new(right_expr));
		}
		expr
	}

	pub fn factor(&mut self) -> Expression {
		let mut expr = self.unary();
		while self.match_token_types(vec![TokenType::SLASH, TokenType::STAR]) {
			let op = self.previous();
			let right_expr = self.unary();
			expr = Expression::BinaryExpression(Box::new(expr), op, Box::new(right_expr));
		}
		expr
	}

	pub fn unary(&mut self) -> Expression {
		if self.match_token_types(vec![TokenType::BANG, TokenType::MINUS]) {
			let op = self.previous();
			let right_expr = self.unary();
			return Expression::Unary(op, Box::new(right_expr));
		}
		return self.primary();
	}

	pub fn primary(&mut self) -> Expression {
		if self.match_token_types(vec![TokenType::LITERAL(Literal::BOOLEAN(true))]) {
			return Expression::Literal(Literal::BOOLEAN(true));
		} else if self.match_token_types(vec![TokenType::LITERAL(Literal::BOOLEAN(false))]) {
			return Expression::Literal(Literal::BOOLEAN(false));
		} else if self.match_token_types(vec![TokenType::LITERAL(Literal::NIL)]) {
			return Expression::Literal(Literal::NIL);
		}

		if self.match_token_types(vec![TokenType::LEFTPAREN]) {
			let expr = self.expression();
			self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
			return Expression::Grouping(Box::new(expr));
		} else {
			return Expression::Literal(Literal::NIL);
		}
	}

	pub fn consume(&self, token: TokenType, err: &str) {
		unimplemented!();
	}

	pub fn match_token_types(&mut self, types: Vec<TokenType>) -> bool {
		for typ in types {
			if self.check(typ) {
				self.advance();
				return true;
			}
		}
		false
	}

	pub fn check(&self, typ: TokenType) -> bool {
		if self.is_at_end() {
			false;
		}
		self.peek().typ == typ
	}

	pub fn peek(&self) -> Token {
		self.tokens.get(self.current_token).unwrap().clone()
	}

	pub fn is_at_end(&self) -> bool {
		self.peek().typ == TokenType::EOF
	}

	pub fn advance(&mut self) -> Token {
		if self.is_at_end() {
			self.current_token = self.current_token + 1;
		}
		self.previous()
	}

	pub fn previous(&self) -> Token {
		self.tokens.get(self.current_token - 1).unwrap().clone()
	}
}
