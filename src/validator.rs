use std::fs::{File, OpenOptions};
use std::io::{Write, Error};

pub struct Validator {
    output_file: String,
    tokens: Vec<String>
}

impl Validator {
    fn write_file(&self) -> Result<File, Error> {
        let mut file: File = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.output_file)?;

        for token in &self.tokens {
            writeln!(file, "{}", token)?;
        }

        Ok(file)
    }
    
    pub fn new(tokens: Vec<String>, output_file: String) -> Validator { Validator { tokens, output_file } }

    // TODO: PENSAR ESTO MEJOR
    pub fn validate(&self) -> Result<File, Error> {
        let file: File = self.write_file()?;
        Ok(file)
    }
}