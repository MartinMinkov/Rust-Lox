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

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, ResolverVariable>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve_statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::BlockStatement(statements) => {
                self.begin_scope();
                self.resolve_statements(statements)?;
                self.end_scope();
                Ok(())
            }
            Statement::VariableDeclaration(identifier, init_expr) => {
                self.declare(identifier);
                if let Some(expr) = &mut *init_expr {
                    self.resolve_expr(expr)?;
                }
                self.define(identifier);
                Ok(())
            }
            Statement::FunctionDeclaration(func) => {
                self.declare(&func.identifier);
                self.declare(&func.identifier);
                self.resolve_function(&func.parameters(), &mut func.body())?;
                Ok(())
            }
            Statement::ExpressionStatement(expr) => {
                self.resolve_expr(expr)?;
                Ok(())
            }
            Statement::IfStatement(condition, then_branch, else_branch) => {
                self.resolve_expr(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(else_branch) = &mut *else_branch {
                    self.resolve_statement(else_branch)?;
                }
                Ok(())
            }
            Statement::PrintStatement(print_expr) => {
                self.resolve_expr(print_expr)?;
                Ok(())
            }
            Statement::ReturnStatement(return_expr) => {
                if let Some(expr) = &mut *return_expr {
                    self.resolve_expr(expr)?;
                }
                Ok(())
            }
            Statement::WhileStatement(condition, body) => {
                self.resolve_expr(condition)?;
                self.resolve_statement(body)?;
                Ok(())
            }
        }
    }

    pub fn resolve_expr(&mut self, expression: &mut ExpressionNode) -> Result<()> {
        match expression.expr_mut() {
            Expression::Variable(ref mut variable) => {
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
                self.resolve_local(variable);
                Ok(())
            }
            Expression::Assignment(variable, assignment_expr) => {
                self.resolve_expr(assignment_expr)?;
                self.resolve_local(variable);
                Ok(())
            }
            Expression::BinaryExpression(left_expr, _, right_expr) => {
                self.resolve_expr(left_expr)?;
                self.resolve_expr(right_expr)?;
                Ok(())
            }
            Expression::CallExpression(callee, _token, args) => {
                self.resolve_expr(callee)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
                Ok(())
            }
            Expression::Grouping(group_expr) => {
                self.resolve_expr(group_expr)?;
                Ok(())
            }
            Expression::Literal(_) => Ok(()),
            Expression::Or(left_expr, _, right_expr)
            | Expression::And(left_expr, _, right_expr) => {
                self.resolve_expr(left_expr)?;
                self.resolve_expr(right_expr)?;
                Ok(())
            }
            Expression::Unary(_, unary_expr) => {
                self.resolve_expr(unary_expr)?;
                Ok(())
            }
            Expression::TernaryExpression(if_expr, _, left_expr, right_expr) => {
                self.resolve_expr(if_expr)?;
                self.resolve_expr(left_expr)?;
                self.resolve_expr(right_expr)?;
                Ok(())
            }
            Expression::FunctionExpression(func) => {
                self.resolve_function(&func.parameters(), &mut func.body())?;
                Ok(())
            }
        }
    }

    pub fn resolve_statements(&mut self, statements: &mut Vec<Statement>) -> Result<()> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_local(&mut self, variable: &mut Variable) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&variable.get_identifier().get_name()) {
                println!("Setting depth for {}", variable.get_identifier().get_name());
                variable.set_depth(i)
            }
        }
    }

    fn resolve_function(
        &mut self,
        params: &Vec<Identifier>,
        body: &mut Vec<Statement>,
    ) -> Result<()> {
        self.begin_scope();
        for param in params {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_statements(body)?;
        self.end_scope();
        Ok(())
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
