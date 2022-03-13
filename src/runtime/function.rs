use super::{Environment, Function, Interpreter, Literal, LoxCallable, Result};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxFunction {
    function: Function,
}

impl LoxFunction {
    pub fn new(function: Function) -> Self {
        Self { function }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal> {
        let environment = Rc::new(RefCell::new(Environment::new_with_environment(
            &interpreter.environment,
        )));

        match &self.function {
            Function::Declaration(func) => {
                for (param, token) in func.parameters.iter().zip(args) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), token.clone())
                }
                let result = interpreter.execute_block(&func.body, environment);
                // TODO: Refactor this
                match result {
                    Ok(return_value) => match return_value {
                        Some(literal) => return Ok(literal),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => println!("unreachable"),
        }
        Ok(Literal::Nil)
    }

    fn arity(&self) -> usize {
        match &self.function {
            Function::Declaration(f) => f.parameters.len(),
            _ => 0,
        }
    }

    fn name(&self) -> &str {
        match &self.function {
            Function::Declaration(f) => f.identifier.lexeme.as_str(),
            _ => "anonymous",
        }
    }
}
