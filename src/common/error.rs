pub use super::{Token, TokenType};

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub line: usize,
}

impl Error {
    pub fn error(line: usize, message: String) {
        Error::report(line, String::from(""), message);
    }

    pub fn report(line: usize, location: String, message: String) {
        println!("[line {} ] Error {} : {}", line, location, message)
    }
}
