use super::{Interpreter, Literal, LoxCallable, LoxInstance, Result};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Literal>) -> Result<Literal> {
        let instance = Rc::new(LoxInstance::new(self.clone()));
        Ok(Literal::Instance(instance))
    }
    fn arity(&self) -> usize {
        0
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}
