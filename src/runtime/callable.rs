use super::Interpreter;
use super::Literal;
use super::Result;
use std::fmt;

pub trait LoxCallable: fmt::Debug {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal>;
    fn arity(&self) -> usize;
    fn name(&self) -> String;
}

impl fmt::Display for dyn LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name())
    }
}
