use super::{LoxCallable, LoxClass};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::string::String;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(Rc<dyn LoxCallable>),
    Class(Rc<LoxClass>),
    Nil,
}

use self::Literal::*;

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &*self {
            Literal::String(val) => write!(f, "{}", val),
            Literal::Number(val) => write!(f, "{}", val),
            Literal::Boolean(val) => write!(f, "{}", val),
            Literal::Callable(val) => write!(f, "{}", val),
            Literal::Class(class) => write!(f, "class {}", class.to_string()),
            Literal::Nil => write!(f, "NIL"),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (&String(ref a), &String(ref b)) => a == b,
            (&Number(a), &Number(b)) => a == b,
            (&Boolean(a), &Boolean(b)) => a == b,
            (&Nil, &Nil) => true,
            (&Callable(ref a), &Callable(ref b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::String(s) => {
                if s.len() == 0 {
                    return false;
                } else {
                    return true;
                }
            }
            Literal::Number(n) => {
                if n <= &(0 as f64) {
                    return false;
                } else {
                    return true;
                }
            }
            Literal::Boolean(b) => *b,
            Literal::Nil | _ => false,
        }
    }

    pub fn into_callable(self) -> Option<Rc<dyn LoxCallable>> {
        match self {
            Literal::Callable(f) => Some(f),
            Literal::Class(_c) => None,
            Literal::String(_) | Literal::Number(_) | Literal::Boolean(_) | Literal::Nil => None,
        }
    }
}
