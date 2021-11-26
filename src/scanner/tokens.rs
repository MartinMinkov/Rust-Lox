use super::Literal;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
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
	NUMBER,
	STRING,
	IDENTIFIER,

	// Keywords.
	NIL,
	FALSE,
	TRUE,
	AND,
	CLASS,
	ELSE,
	FUN,
	FOR,
	IF,
	OR,
	PRINT,
	RETURN,
	SUPER,
	THIS,
	VAR,
	WHILE,

	EOF,
}

impl Display for TokenType {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match &*self {
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
			TokenType::AND => write!(f, "AND"),
			TokenType::CLASS => write!(f, "CLASS"),
			TokenType::ELSE => write!(f, "ELSE"),
			TokenType::FUN => write!(f, "FUN"),
			TokenType::FOR => write!(f, "FOR"),
			TokenType::IF => write!(f, "IF"),
			TokenType::OR => write!(f, "OR"),
			TokenType::PRINT => write!(f, "PRINT"),
			TokenType::RETURN => write!(f, "RETURN"),
			TokenType::SUPER => write!(f, "SUPER"),
			TokenType::THIS => write!(f, "THIS"),
			TokenType::VAR => write!(f, "VAR"),
			TokenType::WHILE => write!(f, "WHILE"),
			TokenType::TRUE => write!(f, "TRUE"),
			TokenType::FALSE => write!(f, "FALSE"),
			TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
			TokenType::STRING => write!(f, "STRING"),
			TokenType::NUMBER => write!(f, "NUMBER"),
			TokenType::NIL => write!(f, "NIL"),
			TokenType::EOF => write!(f, "EOF"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Token {
	pub typ: TokenType,
	pub lexeme: String,
	pub literal: Option<Literal>,
	pub line: u16,
}

impl Token {
	pub fn new(typ: TokenType, lexeme: String, literal: Option<Literal>, line: u16) -> Self {
		Self {
			typ,
			lexeme,
			literal,
			line,
		}
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match &self.literal {
			Some(literal) => write!(f, "{} {}", self.typ, literal),
			None => write!(f, "{}", self.typ),
		}
	}
}
