#[derive(Debug)]
pub struct Variable {
    dir: String,
    name: String,
}

impl Variable {
    pub fn new(dir: &str, name: &str) -> Variable {
        Variable { dir: dir.to_string(), name: name.to_string() }
    }

    pub fn dir(&self) -> &str { &self.dir }
    pub fn name(&self) -> &str { &self.name }
}