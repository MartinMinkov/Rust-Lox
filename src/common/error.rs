pub use super::{Token, TokenType};

#[derive(Debug)]
pub struct Error {
	message: String,
	line: usize,
}

impl Error {
	pub fn error(line: u16, message: String) {
		Error::report(line, String::from(""), message);
	}

	pub fn report(line: u16, location: String, message: String) {
		println!("[line {} ] Error {} : {}", line, location, message)
	}
}
