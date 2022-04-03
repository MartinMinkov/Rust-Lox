use crate::ast::expression::Identifier;

use super::{Error, Expression, ExpressionNode, Result, Statement, Variable};
use std::collections::HashMap;
use std::mem;

pub struct ResolverVariable {
    initialized: bool,
    usages: u32,
    pub line: usize,
}

impl ResolverVariable {
    pub fn unresolved(line: usize) -> Self {
        Self {
            initialized: false,
            usages: 0,
            line,
        }
    }

    pub fn resolved(line: usize) -> Self {
        Self {
            initialized: true,
            usages: 0,
            line,
        }
    }

    pub fn increment_usages(&mut self) {
        self.usages += 1;
    }

    pub fn is_used(&self) -> bool {
        return self.usages > 0;
    }
}

pub enum FunctionKind {
    None,
    Function,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, ResolverVariable>>,
    current_function: FunctionKind,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            current_function: FunctionKind::None,
        }
    }

    pub fn resolve_statement(&mut self, statement: &mut Statement) -> Result<()> {
        match statement {
            Statement::BlockStatement(statements) => {
                self.begin_scope();
                self.resolve_statements(statements)?;
                self.end_scope()?;
            }
            Statement::VariableDeclaration(identifier, init_expr) => {
                self.declare(identifier)?;
                if let Some(expr) = &mut *init_expr {
                    self.resolve_expr(expr)?;
                }
                self.define(identifier);
            }
            Statement::FunctionDeclaration(func) => {
                self.declare(&func.identifier)?;
                self.define(&func.identifier);
                self.resolve_function(&func.parameters, &mut func.body, FunctionKind::Function)?;
            }
            Statement::ClassDeclaration(name, _methods) => {
                self.declare(name)?;
                self.define(name);
            }
            Statement::ExpressionStatement(expr) => {
                self.resolve_expr(expr)?;
            }
            Statement::IfStatement(condition, then_branch, else_branch) => {
                self.resolve_expr(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(else_branch) = &mut *else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Statement::PrintStatement(print_expr) => {
                self.resolve_expr(print_expr)?;
            }
            Statement::ReturnStatement(return_expr) => {
                if let FunctionKind::None = self.current_function {
                    return Err(Error {
                        line: return_expr.as_ref().unwrap().line(),
                        message: String::from("Can't return form top-level code."),
                    });
                };
                if let Some(expr) = &mut *return_expr {
                    self.resolve_expr(expr)?;
                }
            }
            Statement::WhileStatement(condition, body) => {
                self.resolve_expr(condition)?;
                self.resolve_statement(body)?;
            }
        }
        Ok(())
    }

    pub fn resolve_expr(&mut self, expression: &mut ExpressionNode) -> Result<()> {
        match expression.expr_mut() {
            Expression::Variable(ref mut variable) => {
                if let Some(scope) = self.peek_scope() {
                    if let Some(initializer) =
                        scope.get_mut(&mut variable.get_identifier().get_name())
                    {
                        initializer.increment_usages();
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
            Expression::GetExpression(_name, expr) => {
                self.resolve_expr(expr)?;
                Ok(())
            }
            Expression::SetExpression(object, _name, value) => {
                self.resolve_expr(value)?;
                self.resolve_expr(object)?;
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
                self.resolve_function(&func.parameters, &mut func.body, FunctionKind::Function)?;
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
                variable.set_depth(i);
            }
        }
    }

    fn resolve_function(
        &mut self,
        params: &Vec<Identifier>,
        body: &mut Vec<Statement>,
        function_kind: FunctionKind,
    ) -> Result<()> {
        let enclosing_function = mem::replace(&mut self.current_function, function_kind);
        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_statements(body)?;
        self.end_scope()?;
        self.current_function = enclosing_function;
        Ok(())
    }

    fn declare(&mut self, name: &Identifier) -> Result<()> {
        if let Some(scope) = self.peek_scope() {
            if scope.contains_key(&name.get_name()) {
                return Err(Error {
                    line: name.get_line(),
                    message: String::from("Already a variable with this name is in this scope."),
                });
            } else {
                scope.insert(
                    name.get_name(),
                    ResolverVariable::unresolved(name.get_line()),
                );
            }
        };
        Ok(())
    }

    fn define(&mut self, name: &Identifier) {
        if let Some(scope) = self.peek_scope() {
            scope.insert(name.get_name(), ResolverVariable::resolved(name.get_line()));
        };
    }

    fn peek_scope(&mut self) -> Option<&mut HashMap<String, ResolverVariable>> {
        self.scopes.last_mut()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) -> Result<()> {
        // TODO: This code won't work for function declarations. Fix this later.
        // for scope in &self.scopes {
        //     for (name, variable) in scope.iter() {
        //         if !variable.is_used() {
        //             return Err(Error {
        //                 line: variable.line,
        //                 message: format!("Variable \"{}\" has no usages in this scope.", name),
        //             });
        //         }
        //     }
        // }
        self.scopes.pop();
        Ok(())
    }
}
