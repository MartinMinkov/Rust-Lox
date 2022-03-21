use super::{Error, Expression, ExpressionNode, Function, Interpreter, Result, Statement, Token};
use std::collections::HashMap;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn visit_statement(&mut self, statement: Statement) -> Result<()> {
        match &statement {
            Statement::BlockStatement(statements) => {
                self.begin_scope();
                self.resolve_statements(statements);
                self.end_scope();
                Ok(())
            }
            Statement::VariableDeclaration(token, init_expr) => {
                self.declare(&token);
                if init_expr.is_some() {
                    self.resolve_expr(init_expr.as_ref().unwrap());
                }
                self.define(&token);
                Ok(())
            }
            Statement::FunctionDeclaration(func) => {
                self.declare(&func.identifier);
                self.declare(&func.identifier);
                self.resolve_function(func);
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.resolve_expr(expr);
                Ok(())
            }

            Statement::IfStatement(condition, then_branch, else_branch) => {
                self.resolve_expr(&condition);
                self.resolve_statement(&then_branch);
                else_branch.and_then(|else_expr| {
                    self.resolve_statement(&else_expr);
                    Some(else_expr)
                });
                Ok(())
            }

            Statement::PrintStatement(print_expr) => {
                self.resolve_expr(print_expr);
                Ok(())
            }

            Statement::ReturnStatement(_token, return_expr) => {
                return_expr.and_then(|expr| {
                    self.resolve_expr(&expr);
                    Some(expr)
                });
                Ok(())
            }

            Statement::WhileStatement(condition, body) => {
                self.resolve_expr(condition);
                self.resolve_statement(&body);
                Ok(())
            }
        }
    }

    pub fn visit_expression(&mut self, expression: ExpressionNode) -> Result<()> {
        match expression.expression() {
            Expression::Variable(token) => {
                if let Some(scope) = self.peek_scope() {
                    if let Some(initializer) = scope.get(&token.lexeme) {
                        if *initializer == false {
                            return Err(Error {
                                line: token.line,
                                message: String::from(
                                    "Can't read local variable in its own initializer",
                                ),
                            });
                        }
                    }
                }
                self.resolve_local(expression.expression(), token);
                Ok(())
            }

            Expression::Assignment(variable, assignment_expr) => {
                // self.resolve(variable);
                self.resolve_local(assignment_expr.expression(), variable);
                Ok(())
            }

            Expression::BinaryExpression(left_expr, bin_op, right_expr) => {
                self.resolve_expr(left_expr);
                self.resolve_expr(right_expr);
                Ok(())
            }

            Expression::CallExpression(callee, _token, args) => {
                self.resolve_expr(callee);
                for arg in args {
                    self.resolve_expr(arg)
                }
                Ok(())
            }
            Expression::Grouping(group_expr) => {
                self.resolve_expr(group_expr);
                Ok(())
            }

            Expression::Literal(_) => Ok(()),
            Expression::Or(left_expr, operator, right_expr)
            | Expression::And(left_expr, operator, right_expr) => {
                self.resolve_expr(left_expr);
                self.resolve_expr(right_expr);
                Ok(())
            }

            Expression::Unary(unary_op, unary_expr) => {
                self.resolve_expr(unary_expr);
                Ok(())
            }

            Expression::TernaryExpression(if_expr, ternary_op, left_expr, right_expr) => {
                self.resolve_expr(if_expr);
                self.resolve_expr(left_expr);
                self.resolve_expr(right_expr);
                Ok(())
            }

            Expression::FunctionExpression(func) => {
                self.resolve_function(func);
                Ok(())
            }
        }
    }

    pub fn resolve_statements(&self, statements: &Vec<Statement>) {
        for statement in statements {
            self.resolve_statement(statement);
        }
    }

    fn resolve_statement(&self, statement: &Statement) {
        todo!()
    }

    fn resolve_expr(&self, expr: &ExpressionNode) {
        todo!()
    }

    fn resolve_local(&mut self, expr: &Expression, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - i - 1)
            }
        }
    }

    fn resolve_function(&mut self, function: &Function) {
        self.begin_scope();
        for param in function.parameters() {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_statements(&function.body());
        self.end_scope()
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.peek_scope() {
            scope.insert(name.lexeme.clone(), false);
        };
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.peek_scope() {
            scope.insert(name.lexeme.clone(), true);
        };
    }

    fn peek_scope(&mut self) -> Option<&mut HashMap<String, bool>> {
        self.scopes.last_mut()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
