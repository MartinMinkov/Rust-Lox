pub mod interpreter;

use super::common::{Error, Literal, Result};
use super::parsing::{BinaryOperator, Expression, ExpressionNode, TernaryOperator, UnaryOperator};
pub use interpreter::evaluate;
