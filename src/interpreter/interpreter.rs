// use super::Error;
// use super::Expression;
// use super::Literal;
// use super::Result;
// use super::TokenType;

// pub fn evaluate(expr: Expression) -> Result<Literal> {
// 	match expr {
// 		Expression::Literal(val) => Ok(val),
// 		Expression::Unary(op, expr) => {
// 			let value = evaluate(*expr)?;

// 			match op.typ {
// 				TokenType::MINUS => {
// 					if let Literal::NUMBER(n) = value {
// 						Ok(Literal::NUMBER(-n))
// 					} else {
// 						Err(Error {
// 							line: 1,
// 							message: String::from("123"),
// 						})
// 					}
// 				}
// 			}
// 		}
// 	}
// }
