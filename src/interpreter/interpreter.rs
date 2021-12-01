use super::Error;
use super::Literal;
use super::Result;
use super::{BinaryOperator, Expression, ExpressionNode, TernaryOperator, UnaryOperator};

pub fn evaluate(expr_node: ExpressionNode) -> Result<Literal> {
	let expr = expr_node.expression().clone();
	match expr {
		Expression::Literal(val) => Ok(val),
		Expression::Grouping(group_expr) => return evaluate(*group_expr),
		Expression::Unary(unary_op, unary_expr) => {
			let line = unary_expr.line();
			let value = evaluate(*unary_expr)?;
			match unary_op {
				UnaryOperator::MINUS => {
					if let Literal::NUMBER(n) = value {
						Ok(Literal::NUMBER(-n))
					} else {
						Err(Error {
							line: line.into(),
							message: String::from("Value must be a number"),
						})
					}
				}
				UnaryOperator::BANG => {
					if let Literal::BOOLEAN(b) = value {
						Ok(Literal::BOOLEAN(!b))
					} else {
						Err(Error {
							line: line.into(),
							message: String::from("Value must be a boolean"),
						})
					}
				}
			}
		}
		Expression::BinaryExpression(left_expr, bin_op, right_expr) => {
			let line = left_expr.line();
			let left = evaluate(*left_expr)?;
			let right = evaluate(*right_expr)?;
			match bin_op {
				BinaryOperator::PLUS => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 + n2)),
					(Literal::STRING(s1), Literal::STRING(s2)) => {
						Ok(Literal::STRING(format!("{}{}", s1, s2)))
					}
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers or strings"),
					}),
				},
				BinaryOperator::MINUS => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 - n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::SLASH => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 / n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::STAR => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 * n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::GREATER => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 > n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::GREATEREQUAL => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 >= n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::LESS => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 < n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::LESSEQUAL => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 <= n2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::BANGEQUAL => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 != n2)),
					(Literal::BOOLEAN(b1), Literal::BOOLEAN(b2)) => Ok(Literal::BOOLEAN(b1 != b2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::EQUALEQUAL => match (left, right) {
					(Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 == n2)),
					(Literal::BOOLEAN(b1), Literal::BOOLEAN(b2)) => Ok(Literal::BOOLEAN(b1 == b2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
				BinaryOperator::COMMA => match (left, right) {
					(_, Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n2)),
					(_, Literal::STRING(s2)) => Ok(Literal::STRING(s2)),
					(_, Literal::BOOLEAN(b2)) => Ok(Literal::BOOLEAN(b2)),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Values must be numbers"),
					}),
				},
			}
		}
		Expression::TernaryExpression(if_expr, ternary_op, left_expr, right_expr) => {
			let line = if_expr.line();
			let expr = evaluate(*if_expr)?;
			let left = evaluate(*left_expr)?;
			let right = evaluate(*right_expr)?;
			match ternary_op {
				TernaryOperator::QUESTIONMARK => match expr {
					Literal::BOOLEAN(true) => Ok(left),
					Literal::BOOLEAN(false) => Ok(right),
					_ => Err(Error {
						line: line.into(),
						message: String::from("Expression must evaluate to boolean"),
					}),
				},
			}
		}
	}
}
