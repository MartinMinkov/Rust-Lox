#[derive(Debug)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_string(&self) -> &String {
        &self.name
    }
}
