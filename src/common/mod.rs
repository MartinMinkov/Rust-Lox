#[macro_use]
pub mod error;
pub mod literal;
pub use literal::Literal;

pub use super::scanner::{Token, TokenType};
pub use error::Error;
pub type Result<T> = ::std::result::Result<T, Error>;
