pub mod interpreter;

use super::common::{Error, Literal, Result};
use super::parsing::{
	BinaryOperator, Expression, ExpressionNode, Statement, TernaryOperator, UnaryOperator,
};
pub use interpreter::evaluate_statement;
