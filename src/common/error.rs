pub use super::{Token, TokenType};

#[derive(Debug)]
pub struct Error;

impl Error {
	pub fn report_parse_error(token: Token, message: String) {
		if token.typ == TokenType::EOF {
			Error::report(token.line, String::from(" at the end"), message);
		} else {
			Error::report(token.line, format!(" at '{}'", token.lexeme), message);
		}
	}

	pub fn error(line: u16, message: String) {
		Error::report(line, String::from(""), message);
	}

	pub fn report(line: u16, location: String, message: String) {
		println!("[line {} ] Error {} : {}", line, location, message)
	}
}
