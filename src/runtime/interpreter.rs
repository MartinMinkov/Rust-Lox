use super::Environment;
use super::Error;
use super::Literal;
use super::Result;
use super::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

type StatementResult = Option<Literal>;

impl Interpreter {
    pub fn new() -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment
            .borrow_mut()
            .define(Clock.name().into(), Literal::Callable(Rc::new(Clock)));
        Self { environment }
    }

    pub fn evaluate_statement(
        &mut self,
        statement: &Statement,
        run_in_repl: bool,
    ) -> Result<StatementResult> {
        match statement {
            Statement::ReturnStatement(return_expr) => match return_expr {
                Some(return_expr) => match self.evaluate(&*return_expr) {
                    Ok(return_value) => Ok(Some(return_value)),
                    Err(e) => Err(e),
                },
                None => Ok(None),
            },
            Statement::PrintStatement(print_expr) => {
                println!("{}", self.evaluate(&*print_expr)?);
                Ok(None)
            }
            Statement::IfStatement(condition, then_branch, else_branch) => {
                let condition_expr = self.evaluate(&*condition)?;
                if Literal::is_truthy(&condition_expr) {
                    return self.evaluate_statement(&*then_branch, run_in_repl);
                } else {
                    return match else_branch {
                        Some(else_expr) => self.evaluate_statement(&*else_expr, run_in_repl),
                        None => return Ok(None),
                    };
                }
            }
            Statement::WhileStatement(condition, body) => {
                let mut condition_expr = self.evaluate(&*condition)?;
                while Literal::is_truthy(&condition_expr) {
                    self.evaluate_statement(body, run_in_repl)?;
                    condition_expr = self.evaluate(&*condition)?;
                }
                Ok(None)
            }
            Statement::ExpressionStatement(expr) => {
                self.evaluate(&*expr)?;
                Ok(None)
            }
            Statement::VariableDeclaration(id, init_expr) => match init_expr {
                Some(expr) => {
                    let value = self.evaluate(&*expr)?;
                    self.environment.borrow_mut().define(id.get_name(), value);
                    Ok(None)
                }
                None => {
                    self.environment
                        .borrow_mut()
                        .define(id.get_name(), Literal::Nil);
                    Ok(None)
                }
            },
            Statement::FunctionDeclaration(func) => {
                let closure = Rc::new(RefCell::new(Environment::new_with_environment(
                    &self.environment,
                )));
                let f = LoxFunction::new(Function::Declaration(func.clone()), closure);
                self.environment
                    .borrow_mut()
                    .define(f.name().into(), Literal::Callable(Rc::new(f)));
                Ok(None)
            }
            Statement::BlockStatement(statements) => {
                match self.execute_block(
                    statements,
                    Rc::new(RefCell::new(Environment::new_with_environment(
                        &self.environment,
                    ))),
                ) {
                    Ok(block) => Ok(block),
                    Err(_) => Ok(None),
                }
            }
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Statement>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<StatementResult> {
        let previous = self.environment.clone();
        self.environment = environment;
        let mut return_result = Ok(None);
        for statement in statements {
            match self.evaluate_statement(&statement, false) {
                Ok(return_value) => match return_value {
                    Some(literal) => {
                        return_result = Ok(Some(literal));
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        self.environment = previous;
        return_result
    }

    pub fn evaluate(&mut self, expr_node: &ExpressionNode) -> Result<Literal> {
        let line = expr_node.line();
        match expr_node.expr() {
            Expression::FunctionExpression(func) => {
                let callable =
                    LoxFunction::new(Function::Expression(func.clone()), self.environment.clone());
                Ok(Literal::Callable(Rc::new(callable)))
            }
            Expression::CallExpression(callee, _token, args) => {
                let callee_expr = self.evaluate(&*callee)?;
                let args_expr: Vec<Literal> = args
                    .iter()
                    .map(|arg_expr| self.evaluate(&arg_expr).unwrap())
                    .collect();

                match callee_expr.into_callable() {
                    Some(function) => {
                        if args.len() != function.arity() {
                            return Err(Error {
                                line: line.into(),
                                message: format!(
                                    "Expected {} arguments but got {}.",
                                    function.arity(),
                                    args.len()
                                ),
                            });
                        }
                        function.call(self, args_expr)
                    }
                    None => Err(Error {
                        line: line.into(),
                        message: String::from("Can only call functions and classes."),
                    }),
                }
            }
            Expression::Literal(val) => Ok(val.clone()),
            Expression::Grouping(group_expr) => return self.evaluate(&*group_expr),
            Expression::Unary(unary_op, unary_expr) => {
                let line = unary_expr.line();
                let value = self.evaluate(&*unary_expr)?;
                match unary_op {
                    UnaryOperator::MINUS => {
                        if let Literal::Number(n) = value {
                            Ok(Literal::Number(-n))
                        } else {
                            Err(Error {
                                line: line.into(),
                                message: String::from("Operand must be a number."),
                            })
                        }
                    }
                    UnaryOperator::BANG => {
                        if let Literal::Boolean(b) = value {
                            Ok(Literal::Boolean(!b))
                        } else {
                            Err(Error {
                                line: line.into(),
                                message: String::from("Operand must be a boolean."),
                            })
                        }
                    }
                }
            }
            Expression::Assignment(id, assignment_expr) => {
                let line = assignment_expr.line();
                let value = self.evaluate(&*assignment_expr)?;
                let variable_name = id.get_identifier().get_name();
                self.environment
                    .borrow_mut()
                    .assign(variable_name, value)
                    .ok_or_else(|| Error {
                        line: line.into(),
                        message: String::from(format!(
                            "Undefined {} variable.",
                            id.get_identifier().get_name()
                        )),
                    })
            }
            Expression::BinaryExpression(left_expr, bin_op, right_expr) => {
                let line = left_expr.line();
                let left = self.evaluate(&*left_expr)?;
                let right = self.evaluate(&*right_expr)?;
                match bin_op {
                    BinaryOperator::PLUS => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number(n1 + n2)),
                        (Literal::String(s1), Literal::String(s2)) => {
                            Ok(Literal::String(format!("{}{}", s1, s2)))
                        }
                        (Literal::Number(n), Literal::String(s)) => {
                            Ok(Literal::String(format!("{}{}", n, s)))
                        }
                        (Literal::String(s), Literal::Number(n)) => {
                            Ok(Literal::String(format!("{}{}", s, n)))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers or strings."),
                        }),
                    },
                    BinaryOperator::MINUS => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number(n1 - n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::SLASH => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => {
                            if n2 == 0.0 {
                                return Err(Error {
                                    line: line.into(),
                                    message: String::from("Cannot divide by zero."),
                                });
                            } else {
                                return Ok(Literal::Number(n1 / n2));
                            };
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::STAR => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number(n1 * n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::GREATER => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Boolean(n1 > n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::GREATEREQUAL => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => {
                            Ok(Literal::Boolean(n1 >= n2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::LESS => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Boolean(n1 < n2)),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::LESSEQUAL => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => {
                            Ok(Literal::Boolean(n1 <= n2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be numbers."),
                        }),
                    },
                    BinaryOperator::BANGEQUAL => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => {
                            Ok(Literal::Boolean(n1 != n2))
                        }
                        (Literal::Boolean(b1), Literal::Boolean(b2)) => {
                            Ok(Literal::Boolean(b1 != b2))
                        }
                        (Literal::String(s1), Literal::String(s2)) => {
                            Ok(Literal::Boolean(s1 != s2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be strings, numbers or booleans."),
                        }),
                    },
                    BinaryOperator::EQUALEQUAL => match (left, right) {
                        (Literal::Number(n1), Literal::Number(n2)) => {
                            Ok(Literal::Boolean(n1 == n2))
                        }
                        (Literal::Boolean(b1), Literal::Boolean(b2)) => {
                            Ok(Literal::Boolean(b1 == b2))
                        }
                        (Literal::String(s1), Literal::String(s2)) => {
                            Ok(Literal::Boolean(s1 == s2))
                        }
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Operands must be strings, numbers or booleans."),
                        }),
                    },
                    BinaryOperator::COMMA => match (left, right) {
                        (_, Literal::Number(n2)) => Ok(Literal::Number(n2)),
                        (_, Literal::String(s2)) => Ok(Literal::String(s2)),
                        (_, Literal::Boolean(b2)) => Ok(Literal::Boolean(b2)),
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
                        Literal::Boolean(true) => Ok(left),
                        Literal::Boolean(false) => Ok(right),
                        _ => Err(Error {
                            line: line.into(),
                            message: String::from("Expression must evaluate to boolean"),
                        }),
                    },
                }
            }
            Expression::Variable(id) => match self
                .environment
                .borrow()
                .get(id.get_identifier().get_name())
            {
                Some(variable) => Ok(variable),
                _ => Err(Error {
                    line: id.get_identifier().get_line(),
                    message: String::from("Variable must be defined before referenced"),
                }),
            },
            Expression::Or(left_expr, operator, right_expr)
            | Expression::And(left_expr, operator, right_expr) => {
                let left = self.evaluate(&*left_expr)?;
                match operator {
                    LogicalOperator::OR => {
                        if Literal::is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    LogicalOperator::AND => {
                        if !Literal::is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                }
                return self.evaluate(&*right_expr);
            }
        }
    }
}
