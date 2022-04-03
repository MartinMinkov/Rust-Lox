use crate::ast::expression::Identifier;

use super::{Literal, LoxClass};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LoxInstance {
    class: LoxClass,
    fields: RefCell<HashMap<String, Literal>>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> Self {
        Self {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn to_string(&self) -> String {
        self.class.to_string()
    }

    pub fn get(&self, name: &Identifier) -> Option<Literal> {
        self.fields.borrow().get(&name.get_name()).cloned()
    }

    pub fn set(&self, field: String, value: Literal) {
        self.fields.borrow_mut().insert(field, value);
    }
}
