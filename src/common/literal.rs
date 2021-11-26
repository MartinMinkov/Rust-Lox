use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
	FLOAT(f64),
	INTEGER(i64),
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match &*self {
			Number::FLOAT(val) => write!(f, "{}", val),
			Number::INTEGER(val) => write!(f, "{}", val),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	STRING(String),
	NUMBER(Number),
	BOOLEAN(bool),
	NIL,
}

impl Display for Literal {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match &*self {
			Literal::STRING(val) => write!(f, "{}", val),
			Literal::NUMBER(val) => write!(f, "{}", val),
			Literal::BOOLEAN(val) => write!(f, "{}", val),
			Literal::NIL => write!(f, "NIL"),
		}
	}
}
