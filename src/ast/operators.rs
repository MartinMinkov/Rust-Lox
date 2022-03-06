use super::TokenType;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub trait OperatorTokenType {
    fn token_type(&self) -> TokenType;
}

#[derive(Copy, Clone, Debug)]
pub enum LogicalOperator {
    AND,
    OR,
}

impl OperatorTokenType for LogicalOperator {
    fn token_type(&self) -> TokenType {
        match *self {
            LogicalOperator::AND => TokenType::AND,
            LogicalOperator::OR => TokenType::OR,
        }
    }
}

impl Display for LogicalOperator {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            LogicalOperator::AND => write!(f, "and"),
            LogicalOperator::OR => write!(f, "or"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOperator {
    MINUS,
    BANG,
}

impl OperatorTokenType for UnaryOperator {
    fn token_type(&self) -> TokenType {
        match *self {
            UnaryOperator::MINUS => TokenType::MINUS,
            UnaryOperator::BANG => TokenType::BANG,
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            UnaryOperator::MINUS => write!(f, "-"),
            UnaryOperator::BANG => write!(f, "!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
    MINUS,
    PLUS,
    BANGEQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    SLASH,
    STAR,
    COMMA,
}

impl OperatorTokenType for BinaryOperator {
    fn token_type(&self) -> TokenType {
        match *self {
            BinaryOperator::MINUS => TokenType::MINUS,
            BinaryOperator::PLUS => TokenType::PLUS,
            BinaryOperator::BANGEQUAL => TokenType::BANGEQUAL,
            BinaryOperator::EQUALEQUAL => TokenType::EQUALEQUAL,
            BinaryOperator::GREATER => TokenType::GREATER,
            BinaryOperator::GREATEREQUAL => TokenType::GREATEREQUAL,
            BinaryOperator::LESS => TokenType::LESS,
            BinaryOperator::LESSEQUAL => TokenType::LESSEQUAL,
            BinaryOperator::SLASH => TokenType::SLASH,
            BinaryOperator::STAR => TokenType::STAR,
            BinaryOperator::COMMA => TokenType::COMMA,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            BinaryOperator::MINUS => write!(f, "-"),
            BinaryOperator::PLUS => write!(f, "+"),
            BinaryOperator::BANGEQUAL => write!(f, "!="),
            BinaryOperator::EQUALEQUAL => write!(f, "=="),
            BinaryOperator::GREATER => write!(f, ">"),
            BinaryOperator::GREATEREQUAL => write!(f, ">="),
            BinaryOperator::LESS => write!(f, "<"),
            BinaryOperator::LESSEQUAL => write!(f, "<="),
            BinaryOperator::SLASH => write!(f, "/"),
            BinaryOperator::STAR => write!(f, "*"),
            BinaryOperator::COMMA => write!(f, ","),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TernaryOperator {
    QUESTIONMARK,
}

impl OperatorTokenType for TernaryOperator {
    fn token_type(&self) -> TokenType {
        match *self {
            TernaryOperator::QUESTIONMARK => TokenType::QUESTIONMARK,
        }
    }
}

impl Display for TernaryOperator {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            TernaryOperator::QUESTIONMARK => write!(f, "?"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CallOperator {
    LEFTPAREN,
    RIGHTPAREN,
    COMMA,
}

impl OperatorTokenType for CallOperator {
    fn token_type(&self) -> TokenType {
        match *self {
            CallOperator::LEFTPAREN => TokenType::LEFTPAREN,
            CallOperator::RIGHTPAREN => TokenType::RIGHTPAREN,
            CallOperator::COMMA => TokenType::COMMA,
        }
    }
}
