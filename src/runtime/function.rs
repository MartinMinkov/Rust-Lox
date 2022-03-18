use super::{Environment, Function, Interpreter, Literal, LoxCallable, Result, Statement, Token};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxFunction {
    function: Function,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(function: Function, closure: Rc<RefCell<Environment>>) -> Self {
        Self { function, closure }
    }

    fn parameters(&self) -> Vec<Token> {
        match &self.function {
            Function::Declaration(func) => func.parameters.clone(),
            Function::Expression(func) => func.parameters.clone(),
        }
    }

    fn body(&self) -> Vec<Statement> {
        match &self.function {
            Function::Declaration(func) => func.body.clone(),
            Function::Expression(func) => func.body.clone(),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal> {
        let environment = Rc::new(RefCell::new(Environment::new_with_environment(
            &self.closure,
        )));

        for (parameter, value) in self.parameters().iter().zip(args.iter()) {
            environment
                .borrow_mut()
                .define(parameter.lexeme.clone(), value.clone())
        }
        let result = interpreter.execute_block(&self.body(), environment);
        match result {
            Ok(return_value) => match return_value {
                Some(literal) => return Ok(literal),
                _ => {}
            },
            _ => {}
        }
        Ok(Literal::Nil)
    }

    fn arity(&self) -> usize {
        match &self.function {
            Function::Declaration(f) => f.parameters.len(),
            Function::Expression(f) => f.parameters.len(),
        }
    }

    fn name(&self) -> &str {
        match &self.function {
            Function::Declaration(f) => f.identifier.lexeme.as_str(),
            _ => "anonymous",
        }
    }
}
