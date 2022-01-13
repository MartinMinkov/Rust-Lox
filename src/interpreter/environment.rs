use super::Literal;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
	values: HashMap<String, Literal>,
	enclosing: Option<Box<Environment>>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			values: HashMap::new(),
			enclosing: None,
		}
	}

	pub fn new_with_environment(environment: Environment) -> Self {
		Self {
			values: HashMap::new(),
			enclosing: Some(Box::new(environment)),
		}
	}

	pub fn define(&mut self, name: String, value: Literal) {
		self.values.insert(name, value);
	}

	pub fn get(&self, name: String) -> Option<Literal> {
		match self.values.get(&name) {
			Some(variable) => Some(variable.clone()),
			None => match &self.enclosing {
				None => {
					println!("could not find in get for {}", name);
					None
				}
				Some(enclosing) => enclosing.get(name),
			},
		}
	}

	pub fn assign(&mut self, name: String, value: Literal) -> Option<Literal> {
		match self.values.insert(name.clone(), value.clone()) {
			Some(variable) => Some(variable),
			None => match &mut self.enclosing {
				None => {
					println!("could not find in assign for {}", name);
					None
				}
				Some(enclosing) => enclosing.assign(name, value),
			},
		}
	}

	#[allow(dead_code)]
	pub fn print(&self) {
		for (key, value) in &self.values {
			println!("{} = {}", key, value);
		}
	}
}
