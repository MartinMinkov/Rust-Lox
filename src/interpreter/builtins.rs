use super::{Interpreter, Literal, LoxCallable, Result};
use chrono::offset::Utc;

#[derive(Debug)]
pub struct Clock;

impl LoxCallable for Clock {
    fn call(&self, _: &mut Interpreter, _: Vec<Literal>) -> Result<Literal> {
        Ok(Literal::Number(Utc::now().timestamp() as f64))
    }

    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> &str {
        "clock"
    }
}
