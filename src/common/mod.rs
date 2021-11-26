pub mod error;
pub mod literal;
pub use literal::{Literal, Number};

pub use super::scanner::{Token, TokenType};
pub use error::Error;
