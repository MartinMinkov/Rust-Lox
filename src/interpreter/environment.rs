use super::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    pub enclosing: Option<Box<Self>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_environment(environment: Box<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(environment),
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
                |enclosing| enclosing.get(name.clone()),
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
                Some(enclosing) => return enclosing.assign(name, value),
            }
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for (key, value) in &self.values {
            println!("{} = {}", key, value);
        }
    }
}
