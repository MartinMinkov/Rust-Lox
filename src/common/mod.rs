pub mod error;
pub mod literal;

pub use super::parsing::UnaryOperator;
pub use super::scanner::{Token, TokenType};
pub use literal::Literal;

pub use error::Error;
pub type Result<T> = ::std::result::Result<T, Error>;
