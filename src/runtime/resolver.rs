use crate::ast::expression::Identifier;

use super::{
    Error, Expression, ExpressionNode, FunctionInfo, Interpreter, Result, Statement, Variable,
};
use std::collections::HashMap;

pub struct ResolverVariable {
    initialized: bool,
}

impl ResolverVariable {
    pub fn unresolved() -> Self {
        Self { initialized: false }
    }

    pub fn resolved() -> Self {
        Self { initialized: true }
    }
}

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, ResolverVariable>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::BlockStatement(statements) => {
                self.begin_scope();
                self.resolve_statements(statements);
                self.end_scope();
                Ok(())
            }
            Statement::VariableDeclaration(id, init_expr) => {
                self.declare(id);
                if init_expr.is_some() {
                    self.resolve_expr(init_expr.as_ref().unwrap());
                }
                self.define(id);
                Ok(())
            }
            Statement::FunctionDeclaration(func) => {
                self.declare(&func.identifier);
                self.declare(&func.identifier);
                self.resolve_function(func.parameters(), func.body());
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.resolve_expr(expr);
                Ok(())
            }
            Statement::IfStatement(condition, then_branch, else_branch) => {
                self.resolve_expr(&condition);
                self.resolve_statement(&then_branch);
                else_branch.as_ref().and_then(|else_expr| {
                    self.resolve_statement(&else_expr);
                    Some(else_expr)
                });
                Ok(())
            }
            Statement::PrintStatement(print_expr) => {
                self.resolve_expr(print_expr);
                Ok(())
            }
            Statement::ReturnStatement(return_expr) => {
                return_expr.as_ref().and_then(|expr| {
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

    pub fn resolve_expr(&mut self, expression: &ExpressionNode) -> Result<()> {
        match expression.expression() {
            Expression::Variable(variable) => {
                if let Some(scope) = self.peek_scope() {
                    if let Some(initializer) = scope.get(&variable.get_identifier().get_name()) {
                        if !initializer.initialized {
                            return Err(Error {
                                line: variable.get_identifier().get_line(),
                                message: String::from(
                                    "Can't read local variable in its own initializer",
                                ),
                            });
                        }
                    }
                }
                self.resolve_local(expression.expression(), variable);
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
                    self.resolve_expr(arg);
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
                self.resolve_function(func.parameters(), func.body());
                Ok(())
            }
        }
    }

    pub fn resolve_statements(&mut self, statements: &Vec<Statement>) {
        for statement in statements {
            self.resolve_statement(statement);
        }
    }

    fn resolve_local(&mut self, expr: &Expression, variable: &Variable) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&variable.get_identifier().get_name()) {
                self.interpreter.resolve(expr, self.scopes.len() - i - 1);
            }
        }
    }

    fn resolve_function(&mut self, params: Vec<Identifier>, body: Vec<Statement>) {
        self.begin_scope();
        for param in params {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_statements(&body);
        self.end_scope()
    }

    fn declare(&mut self, name: &Identifier) {
        if let Some(scope) = self.peek_scope() {
            scope.insert(name.get_name(), ResolverVariable::unresolved());
        };
    }

    fn define(&mut self, name: &Identifier) {
        if let Some(scope) = self.peek_scope() {
            scope.insert(name.get_name(), ResolverVariable::resolved());
        };
    }

    fn peek_scope(&mut self) -> Option<&mut HashMap<String, ResolverVariable>> {
        self.scopes.last_mut()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}
