use super::Literal;
use std::collections::HashMap;

pub struct Environment {
	values: HashMap<String, Literal>,
}

impl Environment {
	pub fn new() -> Self {
		Self {
			values: HashMap::new(),
		}
	}

	pub fn define(&mut self, name: String, value: Literal) {
		self.values.insert(name, value);
	}

	pub fn get(&self, name: String) -> Option<Literal> {
		self.values.get(&name).map(|variable| variable.clone())
	}

	pub fn print(&self) {
		for (key, value) in &self.values {
			println!("{} = {}", key, value);
		}
	}
}
