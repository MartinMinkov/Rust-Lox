use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
	// Single-character tokens.
	LEFTPAREN,
	RIGHTPAREN,
	LEFTBRACE,
	RIGHTBRACE,
	COMMA,
	DOT,
	MINUS,
	PLUS,
	SEMICOLON,
	SLASH,
	STAR,

	// One or two character tokens.
	BANG,
	BANGEQUAL,
	EQUAL,
	EQUALEQUAL,
	GREATER,
	GREATEREQUAL,
	LESS,
	LESSEQUAL,

	// Literals.
	IDENTIFIER,
	STRING,
	NUMBER,

	// Keywords.
	AND,
	CLASS,
	ELSE,
	FALSE,
	FUN,
	FOR,
	IF,
	NIL,
	OR,
	PRINT,
	RETURN,
	SUPER,
	THIS,
	TRUE,
	VAR,
	WHILE,

	EOF,
}

#[derive(Debug)]
pub struct Token {
	pub typ: TokenType,
	lexeme: String,
	literal: String,
	line: u16,
}

impl Token {
	pub fn new(typ: TokenType, lexeme: String, literal: String, line: u16) -> Self {
		Self {
			typ,
			lexeme,
			literal,
			line,
		}
	}
}

impl Display for TokenType {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match *self {
			TokenType::LEFTPAREN => write!(f, "("),
			TokenType::RIGHTPAREN => write!(f, ")"),
			TokenType::LEFTBRACE => write!(f, "{{"),
			TokenType::RIGHTBRACE => write!(f, "}}"),
			TokenType::COMMA => write!(f, ","),
			TokenType::DOT => write!(f, "."),
			TokenType::MINUS => write!(f, "-"),
			TokenType::PLUS => write!(f, "+"),
			TokenType::SEMICOLON => write!(f, ";"),
			TokenType::SLASH => write!(f, "/"),
			TokenType::STAR => write!(f, "*"),
			TokenType::BANG => write!(f, "!"),
			TokenType::BANGEQUAL => write!(f, "!="),
			TokenType::EQUAL => write!(f, "="),
			TokenType::EQUALEQUAL => write!(f, "=="),
			TokenType::GREATER => write!(f, ">"),
			TokenType::GREATEREQUAL => write!(f, ">="),
			TokenType::LESS => write!(f, "<"),
			TokenType::LESSEQUAL => write!(f, "<="),
			TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
			TokenType::STRING => write!(f, "STRING"),
			TokenType::NUMBER => write!(f, "NUMBER"),
			TokenType::AND => write!(f, "AND"),
			TokenType::CLASS => write!(f, "CLASS"),
			TokenType::ELSE => write!(f, "ELSE"),
			TokenType::FALSE => write!(f, "FALSE"),
			TokenType::FUN => write!(f, "FUN"),
			TokenType::FOR => write!(f, "FOR"),
			TokenType::IF => write!(f, "IF"),
			TokenType::NIL => write!(f, "NIL"),
			TokenType::OR => write!(f, "OR"),
			TokenType::PRINT => write!(f, "PRINT"),
			TokenType::RETURN => write!(f, "RETURN"),
			TokenType::SUPER => write!(f, "SUPER"),
			TokenType::THIS => write!(f, "THIS"),
			TokenType::TRUE => write!(f, "TRUE"),
			TokenType::VAR => write!(f, "VAR"),
			TokenType::WHILE => write!(f, "WHILE"),
			TokenType::EOF => write!(f, "EOF"),
			_ => write!(f, "NOT_FOUND"),
		}
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(
			f,
			"{} {} {} {}",
			self.typ, self.lexeme, self.literal, self.line
		)
	}
}
