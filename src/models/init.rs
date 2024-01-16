/// Represents the initialization section of a program.
#[derive(Debug)]
pub struct Init {
    dir: String,
}

impl Init {
    pub fn new(dir: &str) -> Init {
        Init { dir: dir.to_string() }
    }

    pub fn dir(&self) -> &str {
        &self.dir
    }
}