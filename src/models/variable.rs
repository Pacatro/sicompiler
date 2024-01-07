/// Represents a variable in a program with a directory and a name.
#[derive(Debug)]
pub struct Variable {
    dir: String,
    name: String,
}

impl Variable {
    /// Creates a new `Variable` with the specified directory and name.
    ///
    /// ## Arguments
    ///
    /// - `dir` - The directory of the variable.
    /// - `name` - The name of the variable.
    ///
    /// ## Returns
    ///
    /// A new `Variable` instance.
    pub fn new(dir: &str, name: &str) -> Variable {
        Variable { dir: dir.to_string(), name: name.to_string() }
    }

    pub fn dir(&self) -> &str { &self.dir }
    pub fn name(&self) -> &str { &self.name }
}