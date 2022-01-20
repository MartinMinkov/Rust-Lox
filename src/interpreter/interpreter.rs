use super::Environment;
use super::Error;
use super::Literal;
use super::Result;
use super::*;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn evaluate_statement(&mut self, statement: &Statement, run_in_repl: bool) -> Result<()> {
        match statement {
            Statement::PrintStatement(print_expr) => {
                println!("{}", self.evaluate(&*print_expr)?);
                Ok(())
            }
            Statement::IfStatement(condition, then_branch, else_branch) => {
                let condition_expr = self.evaluate(&*condition)?;
                if is_truthy(&condition_expr) {
                    self.evaluate_statement(&*then_branch, run_in_repl)?;
                } else {
                    else_branch.as_ref().and_then(|else_expr| {
                        Some(self.evaluate_statement(&*else_expr, run_in_repl))
                    });
                }
                Ok(())
            }
            Statement::WhileStatement(condition, body) => {
                let mut condition_expr = self.evaluate(&*condition)?;
                while is_truthy(&condition_expr) {
                    self.evaluate_statement(body, run_in_repl)?;
                    condition_expr = self.evaluate(&*condition)?;
                }
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.evaluate(&*expr)?;
                Ok(())
            }
            Statement::VariableDeclaration(token, init_expr) => match init_expr {
                Some(expr) => {
                    let value = self.evaluate(&*expr)?;
                    self.environment.define(token.lexeme.clone(), value);
                    Ok(())
                }
                None => {
                    self.environment.define(token.lexeme.clone(), Literal::NIL);
                    Ok(())
                }
            },
            Statement::BlockStatement(statements) => {
                let env = Environment::new_with_environment(Box::new(self.environment.clone()));
                self.execute_block(statements, env);
                Ok(())
            }
        }
    }

    fn execute_block(&mut self, statements: &Vec<Statement>, environment: Environment) {
        self.environment = environment;
        for statement in statements {
            let _ = self.evaluate_statement(&statement, false);
        }
        self.environment = *self.environment.clone().enclosing.unwrap();
    }

    pub fn evaluate(&mut self, expr_node: &ExpressionNode) -> Result<Literal> {
        let expr = expr_node.expression().clone();
        match expr {
            Expression::Literal(val) => Ok(val),
            Expression::Grouping(group_expr) => return self.evaluate(&*group_expr),
            Expression::Unary(unary_op, unary_expr) => {
                let line = unary_expr.line();
                let value = self.evaluate(&*unary_expr)?;
                match unary_op {
                    UnaryOperator::MINUS => {
                        if let Literal::NUMBER(n) = value {
                            Ok(Literal::NUMBER(-n))
                        } else {
                            Err(Error {
                                line: line.into(),
                                message: String::from("Operand must be a number."),
                            })
                        }
                    }
                    UnaryOperator::BANG => {
                        if let Literal::BOOLEAN(b) = value {
                            Ok(Literal::BOOLEAN(!b))
                        } else {
                            Err(Error {
                                line: line.into(),
                                message: String::from("Operand must be a boolean."),
                            })
                        }
                    }
                }
            }
            Expression::Assignment(variable, assignment_expr) => {
                let line = assignment_expr.line();
                let value = self.evaluate(&*assignment_expr)?;
                let variable_name = variable.lexeme.clone();
                self.environment
                    .assign(variable.lexeme, value)
                    .ok_or_else(|| Error {
                        line: line.into(),
                        message: String::from(format!("Undefined {} variable.", variable_name)),
                    })
            }
            Expression::BinaryExpression(left_expr, bin_op, right_expr) => {
                let line = left_expr.line();
                let left = self.evaluate(&*left_expr)?;
                let right = self.evaluate(&*right_expr)?;
                match bin_op {
                    BinaryOperator::PLUS => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 + n2)),
                        (Literal::STRING(s1), Literal::STRING(s2)) => {
                            Ok(Literal::STRING(format!("{}{}", s1, s2)))
                        }
                        (Literal::NUMBER(n), Literal::STRING(s)) => {
                            Ok(Literal::STRING(format!("{}{}", n, s)))
                        }
                        (Literal::STRING(s), Literal::NUMBER(n)) => {
                            Ok(Literal::STRING(format!("{}{}", s, n)))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers or strings."),
                        }),
                    },
                    BinaryOperator::MINUS => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 - n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::SLASH => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => {
                            if n2 == 0.0 {
                                return Err(Error {
                                    line: line.into(),
                                    message: String::from("Cannot divide by zero."),
                                });
                            } else {
                                return Ok(Literal::NUMBER(n1 / n2));
                            };
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::STAR => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n1 * n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::GREATER => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 > n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::GREATEREQUAL => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => {
                            Ok(Literal::BOOLEAN(n1 >= n2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::LESS => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => Ok(Literal::BOOLEAN(n1 < n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::LESSEQUAL => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => {
                            Ok(Literal::BOOLEAN(n1 <= n2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::BANGEQUAL => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => {
                            Ok(Literal::BOOLEAN(n1 != n2))
                        }
                        (Literal::BOOLEAN(b1), Literal::BOOLEAN(b2)) => {
                            Ok(Literal::BOOLEAN(b1 != b2))
                        }
                        (Literal::STRING(s1), Literal::STRING(s2)) => {
                            Ok(Literal::BOOLEAN(s1 != s2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be strings, numbers or booleans."),
                        }),
                    },
                    BinaryOperator::EQUALEQUAL => match (left, right) {
                        (Literal::NUMBER(n1), Literal::NUMBER(n2)) => {
                            Ok(Literal::BOOLEAN(n1 == n2))
                        }
                        (Literal::BOOLEAN(b1), Literal::BOOLEAN(b2)) => {
                            Ok(Literal::BOOLEAN(b1 == b2))
                        }
                        (Literal::STRING(s1), Literal::STRING(s2)) => {
                            Ok(Literal::BOOLEAN(s1 == s2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be strings, numbers or booleans."),
                        }),
                    },
                    BinaryOperator::COMMA => match (left, right) {
                        (_, Literal::NUMBER(n2)) => Ok(Literal::NUMBER(n2)),
                        (_, Literal::STRING(s2)) => Ok(Literal::STRING(s2)),
                        (_, Literal::BOOLEAN(b2)) => Ok(Literal::BOOLEAN(b2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be strings, numbers or booleans."),
                        }),
                    },
                }
            }
            Expression::TernaryExpression(if_expr, ternary_op, left_expr, right_expr) => {
                let line = if_expr.line();
                let expr = self.evaluate(&*if_expr)?;
                let left = self.evaluate(&*left_expr)?;
                let right = self.evaluate(&*right_expr)?;
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
            Expression::Variable(token) => match self.environment.get(token.lexeme) {
                Some(variable) => Ok(variable),
                _ => Err(Error {
                    line: token.line,
                    message: String::from("Variable must be defined before referenced"),
                }),
            },
            Expression::Or(left_expr, operator, right_expr)
            | Expression::And(left_expr, operator, right_expr) => {
                let left = self.evaluate(&*left_expr)?;
                match operator {
                    LogicalOperator::OR => {
                        if is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    LogicalOperator::AND => {
                        if !is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                }

                return self.evaluate(&*right_expr);
            }
        }
    }
}

pub fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::STRING(s) => {
            if s.len() == 0 {
                return false;
            } else {
                return true;
            }
        }
        Literal::NUMBER(n) => {
            if n <= &(0 as f64) {
                return false;
            } else {
                return true;
            }
        }
        Literal::BOOLEAN(b) => b.clone(),
        Literal::NIL => {
            return false;
        }
    }
}
