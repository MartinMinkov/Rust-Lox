use super::{Environment, Function, FunctionInfo, Interpreter, Literal, LoxCallable, Result};
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
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal> {
        let environment = Rc::new(RefCell::new(Environment::new_with_environment(
            &self.closure,
        )));

        for (parameter, value) in self.function.parameters().iter().zip(args.iter()) {
            environment
                .borrow_mut()
                .define(parameter.get_name().clone(), value.clone())
        }
        let result = interpreter.execute_block(&self.function.body(), environment);
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
        self.function.parameters().len()
    }

    fn name(&self) -> String {
        self.function.identifier()
    }
}
