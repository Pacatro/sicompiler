use std::collections::HashMap;
use std::io::{Write, Error};
use std::fs::{File, OpenOptions};

/// The `Validator` struct is responsible for validating a sequence of tokens
/// representing a custom assembly language. It ensures that each instruction
/// is valid and adheres to the expected format, raising errors if any issues are detected.
pub struct Validator {
    output_file: String,
    tokens: Vec<String>
}

impl Validator {
    fn write_file(&self) -> Result<File, Error> {
        let mut file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.output_file)?;

        for token in &self.tokens {
            writeln!(file, "{}", token)?;
        }

        Ok(file)
    }

    fn get_num_params(&self, instruction: &Vec<&str>) -> usize {
        let digits: Vec<u32> = instruction
            .iter()
            .flat_map(|s| s.chars().filter(|c| c.is_digit(10)))
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u32)
            .collect();

        digits.len()
    }
    
    /// Creates a new `Validator` instance with the specified tokens and output file.
    pub fn new(tokens: Vec<String>, output_file: String) -> Validator { Validator { tokens, output_file } }

    /// Validates the sequence of tokens based on predefined rules.
    /// Returns a `Result` containing the opened file with validated tokens or an error.
    pub fn validate(&self) -> Result<File, Error> {
        let valid_tokens: HashMap<&str, usize> = HashMap::from([
            ("CRA", 0),
            ("CTA", 0),
            ("ITA", 0),
            ("CRF", 0),
            ("CTF", 0),
            ("SFZ", 0),
            ("ROR_F_ACC", 0),
            ("ROL_F_ACC", 0),
            ("ADD", 1),
            ("ADDI", 1),
            ("STA", 1),
            ("JMP", 1),
            ("JMPI", 1),
            ("HALT", 0)
        ]);

        for token in &self.tokens {
            let instruction: Vec<&str> = token.split_whitespace().collect();

            if !valid_tokens.contains_key(&instruction[0]) {
                let msg: String = format!("Invalid instruction --> {}", instruction[0]);
                return Err(Error::new(std::io::ErrorKind::Other, msg));
            }

            if &self.get_num_params(&instruction) != valid_tokens.get(instruction[0]).unwrap() {
                let msg: String = format!("Invalid number of parameters in {}", instruction[0]);
                return Err(Error::new(std::io::ErrorKind::Other, msg));
            }
        }
        
        let file: File = self.write_file()?;
        Ok(file)
    }
}