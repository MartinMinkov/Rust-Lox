use super::Literal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_environment(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<Literal> {
        match self.values.get(&name) {
            Some(variable) => Some(variable.clone()),
            None => self.enclosing.as_ref().map_or_else(
                || {
                    println!("could not find in get for {}", name);
                    None
                },
                |enclosing| enclosing.borrow().get(name.clone()),
            ),
        }
    }

    pub fn assign(&mut self, name: String, value: Literal) -> Option<Literal> {
        if self.values.contains_key(&name) {
            return self.values.insert(name.clone(), value.clone());
        } else {
            match &mut self.enclosing {
                None => {
                    println!("could not find in assign for {}", name);
                    return None;
                }
                Some(enclosing) => return enclosing.borrow_mut().assign(name, value),
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_environment(&self) {
        println!("---------- Printing Values ----------");
        for (key, value) in &self.values {
            println!("{} = {}", key, value);
        }
        println!("---------- Printing Values ----------");
        let mut temp_env = self.enclosing.clone();
        println!("---------- Printing Enclosed ----------");
        while temp_env.is_some() {
            for (key, value) in temp_env.clone().unwrap().borrow().values.clone() {
                println!("{} = {}", key, value);
            }
            temp_env = temp_env
                .clone()
                .as_ref()
                .unwrap()
                .borrow()
                .enclosing
                .clone();
        }
        println!("---------- Printing Enclosed ----------");
        println!();
    }
}
