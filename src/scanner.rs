use super::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
	source: String,
	pub tokens: Vec<Token>,
	start: u16,
	current: u16,
	line: u16,
}

impl Scanner {
	pub fn new(source: String) -> Self {
		Self {
			source,
			tokens: vec![],
			start: 0,
			current: 0,
			line: 1,
		}
	}

	pub fn scan_tokens(&mut self) {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}

		let eof_token = Token::new(
			TokenType::EOF,
			String::from(""),
			String::from(""),
			self.line,
		);
		self.tokens.push(eof_token);
	}

	pub fn is_at_end(&self) -> bool {
		self.current >= self.source.chars().count() as u16
	}

	pub fn scan_token(&mut self) {
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
				} else {
					self.add_token(TokenType::SLASH)
				}
			}
			' ' | '\r' | '\t' => (),
			'\n' => self.line = self.line + 1,
			'"' => self.string(),
			_ => {}
		}
	}

	pub fn advance(&mut self) -> char {
		let c = self.source.chars().nth(self.current.into());
		self.current = self.current + 1;
		c.unwrap()
	}

	pub fn add_token(&mut self, token: TokenType) {
		let start: usize = self.start.into();
		let current: usize = self.current.into();
		let text = &self.source.as_str()[start..current];
		self
			.tokens
			.push(Token::new(token, text.into(), String::from(""), self.line));
	}

	pub fn match_token(&mut self, expected: char) -> bool {
		if self.is_at_end() {
			return false;
		}
		if self.source.chars().nth(self.current.into()).unwrap() != expected {
			return false;
		}
		self.current = self.current + 1;
		true
	}

	pub fn peek(&mut self) -> char {
		if self.is_at_end() {
			return '\0';
		}
		self.source.chars().nth(self.current.into()).unwrap()
	}

	pub fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {
				self.line = self.line + 1;
			}
			self.advance();
		}

		// Skip the trailing '"' from the string
		self.advance();
		let start = self.start + 1;
		let end = self.current - 1;
		let str_token = self.source[start.into()..end.into()].to_string();
		self.add_token(TokenType::STRING(str_token))
	}
}
