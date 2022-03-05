use super::{Environment, Interpreter, Literal, LoxCallable, Result, Statement};

#[derive(Debug)]
pub struct LoxFunction {
    declaration: Statement,
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal> {
        let mut environment = Environment::new_with_environment(Box::new(interpreter.globals()));
        match &self.declaration {
            Statement::FunctionDeclaration(_, params, body) => {
                for (param, token) in params.into_iter().zip(args) {
                    environment.define(param.lexeme.clone(), token.clone())
                }
                interpreter.execute_block(&body, environment)
            }
            _ => println!("unreachable"),
        }
        Ok(Literal::Nil)
    }

    fn arity(&self) -> usize {
        match &self.declaration {
            Statement::FunctionDeclaration(_, params, _) => params.len(),
            _ => 0,
        }
    }

    fn name(&self) -> &str {
        match &self.declaration {
            Statement::FunctionDeclaration(name, _, _) => name.lexeme.as_str(),
            _ => "anonymous",
        }
    }
}
