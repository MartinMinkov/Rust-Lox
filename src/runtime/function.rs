use super::{Environment, Function, Interpreter, Literal, LoxCallable, Result};

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
        let mut environment = Environment::new_with_environment(Box::new(interpreter.globals()));
        match &self.function {
            Function::Declaration(func) => {
                for (param, token) in func.parameters.iter().zip(args) {
                    environment.define(param.lexeme.clone(), token.clone())
                }
                interpreter.execute_block(&func.body, environment)
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