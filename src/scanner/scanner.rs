use super::{Error, Literal, Number, Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
	source: String,
	start: usize,
	current: usize,
	line: u16,
	pub tokens: Vec<Token>,
	pub had_error: bool,
}

impl Scanner {
	pub fn new(source: String) -> Self {
		Self {
			source,
			tokens: vec![],
			start: 0,
			current: 0,
			line: 1,
			had_error: false,
		}
	}

	pub fn scan_tokens(&mut self) {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}

		let eof_token = Token::new(TokenType::EOF, String::from(""), None, self.line);
		self.tokens.push(eof_token);
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.chars().count()
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		match c {
			'(' => self.add_token(TokenType::LEFTPAREN),
			')' => self.add_token(TokenType::RIGHTPAREN),
			'{' => self.add_token(TokenType::LEFTBRACE),
			'}' => self.add_token(TokenType::RIGHTBRACE),
			',' => self.add_token(TokenType::COMMA),
			'.' => self.add_token(TokenType::DOT),
			'-' => self.add_token(TokenType::MINUS),
			'+' => self.add_token(TokenType::PLUS),
			';' => self.add_token(TokenType::SEMICOLON),
			'*' => self.add_token(TokenType::STAR),
			'!' => {
				let token = if self.match_token('=') {
					TokenType::BANGEQUAL
				} else {
					TokenType::BANG
				};
				self.add_token(token)
			}
			'=' => {
				let token = if self.match_token('=') {
					TokenType::EQUALEQUAL
				} else {
					TokenType::EQUAL
				};
				self.add_token(token)
			}
			'<' => {
				let token = if self.match_token('=') {
					TokenType::LESSEQUAL
				} else {
					TokenType::LESS
				};
				self.add_token(token)
			}
			'>' => {
				let token = if self.match_token('=') {
					TokenType::GREATEREQUAL
				} else {
					TokenType::GREATER
				};
				self.add_token(token)
			}
			'/' => {
				if self.match_token('/') {
					// A comment goes until the end of the line
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				} else if self.match_token('*') {
					while !self.is_at_end() {
						self.advance();
						// A block comment goes until */
						while self.peek() == '*' && self.peek_next() == '/' {
							self.advance(); // Consume the */ part of the block comment
						}
					}
				} else {
					self.add_token(TokenType::SLASH)
				}
			}
			' ' | '\r' | '\t' => (),
			'\n' => self.line = self.line + 1,
			'"' => self.string(),
			c => {
				if self.is_digit(c) {
					self.number();
				} else if self.is_alpha(c) {
					self.identifier();
				} else {
					Error::error(self.line, String::from("Unexpected character."));
					self.had_error = true;
				}
			}
		}
	}

	fn advance(&mut self) -> char {
		let c = self.source.chars().nth(self.current.into());
		self.current = self.current + 1;
		c.unwrap()
	}

	fn add_token(&mut self, token: TokenType) {
		let start: usize = self.start;
		let current: usize = self.current;
		let text = &self.source.as_str()[start..current];
		self
			.tokens
			.push(Token::new(token, text.into(), None, self.line));
	}

	fn add_token_literal(&mut self, token: TokenType, literal: Literal) {
		let start: usize = self.start;
		let current: usize = self.current;
		let text = &self.source.as_str()[start..current];
		self
			.tokens
			.push(Token::new(token, text.into(), Some(literal), self.line));
	}

	fn match_token(&mut self, expected: char) -> bool {
		if self.is_at_end() {
			return false;
		}
		if self.source.chars().nth(self.current.into()).unwrap() != expected {
			return false;
		}
		self.current = self.current + 1;
		true
	}

	fn peek(&self) -> char {
		if self.is_at_end() {
			return '\0';
		}
		self.source.chars().nth(self.current.into()).unwrap()
	}

	fn peek_next(&self) -> char {
		let current: usize = (self.current + 1).into();
		if current >= self.source.chars().count() {
			return '\0';
		}
		self.source.chars().nth(current).unwrap()
	}

	fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line = self.line + 1;
			}
			self.advance();
		}
		if self.is_at_end() {
			Error::error(self.line, String::from("Unterminated string."));
			self.had_error = true;
			return;
		}

		// Skip the trailing '"' from the string
		self.advance();
		let start = self.start + 1;
		let end = self.current - 1;
		let string_literal = Literal::STRING(self.source[start.into()..end.into()].to_string());
		self.add_token_literal(TokenType::STRING, string_literal)
	}

	fn number(&mut self) {
		while self.is_digit(self.peek()) {
			self.advance();
		}
		let mut is_float: bool = false;
		if self.peek() == '.' && self.is_digit(self.peek_next()) {
			is_float = true;
			self.advance(); // Consume the '.'
			while self.is_digit(self.peek()) {
				self.advance();
			}
		}
		if is_float {
			let float_token = self.source[self.start.into()..self.current.into()]
				.parse::<f64>()
				.unwrap();
			let float_literal = Literal::NUMBER(Number::FLOAT(float_token));
			self.add_token_literal(TokenType::NUMBER, float_literal);
		} else {
			let int_token = self.source[self.start.into()..self.current.into()]
				.parse::<i64>()
				.unwrap();
			let int_literal = Literal::NUMBER(Number::INTEGER(int_token));
			self.add_token_literal(TokenType::NUMBER, int_literal);
		};
	}

	fn identifier(&mut self) {
		while self.is_alpha_numeric(self.peek()) {
			self.advance();
		}

		let identifier_token = self.source[self.start.into()..self.current.into()].to_string();
		match &identifier_token[..] {
			"and" => self.add_token(TokenType::AND),
			"class" => self.add_token(TokenType::CLASS),
			"else" => self.add_token(TokenType::ELSE),
			"false" => self.add_token(TokenType::FALSE),
			"for" => self.add_token(TokenType::FOR),
			"fun" => self.add_token(TokenType::FUN),
			"if" => self.add_token(TokenType::IF),
			"nil" => self.add_token(TokenType::NIL),
			"or" => self.add_token(TokenType::OR),
			"print" => self.add_token(TokenType::PRINT),
			"return" => self.add_token(TokenType::RETURN),
			"super" => self.add_token(TokenType::SUPER),
			"this" => self.add_token(TokenType::THIS),
			"true" => self.add_token(TokenType::TRUE),
			"var" => self.add_token(TokenType::VAR),
			"while" => self.add_token(TokenType::WHILE),
			_ => self.add_token(TokenType::IDENTIFIER),
		}
	}

	fn is_digit(&self, c: char) -> bool {
		c >= '0' && c <= '9'
	}

	fn is_alpha(&self, c: char) -> bool {
		(c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
	}

	fn is_alpha_numeric(&self, c: char) -> bool {
		self.is_alpha(c) || self.is_digit(c)
	}
}
