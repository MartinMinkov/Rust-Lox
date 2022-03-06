pub mod error;
pub mod literal;

pub use super::ast::UnaryOperator;
pub use super::runtime::LoxCallable;
pub use super::scanner::{Token, TokenType};
pub use literal::Literal;

pub use error::Error;
pub type Result<T> = ::std::result::Result<T, Error>;
