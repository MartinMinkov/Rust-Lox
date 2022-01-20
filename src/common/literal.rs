use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    STRING(String),
    NUMBER(f64),
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
