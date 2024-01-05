use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, Error, ErrorKind};

/// The `Validator` struct is responsible for validating a sequence of tokens
/// representing a custom assembly language. It ensures that each instruction
/// is valid and adheres to the expected format, raising errors if any issues are detected.
pub struct Validator {
    output_file: String,
    tokens: Vec<String>
}

impl Validator {
    fn write_file(&self) -> Result<(), Error> {
        let mut file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.output_file)?;

        for token in &self.tokens {
            writeln!(file, "{}", token)?;
        }

        Ok(())
    }

    fn valid_params(&self, params: &Vec<&str>) -> bool {
        params.iter().next().unwrap().chars().next().unwrap().is_ascii_hexdigit()
    }
    
    /// Creates a new `Validator` instance with the specified tokens and output file.
    pub fn new(tokens: Vec<String>, output_file: String) -> Validator { Validator { tokens, output_file } }

    /// Validates the sequence of tokens based on predefined rules.
    /// 
    /// # Arguments
    /// 
    /// - `self` - A reference to the `Validator` instance.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` with a unit type `()` indicating success or an `Error` if validation fails.
    /// 
    /// # Errors
    /// 
    /// The function may return an error in the following cases:
    /// 
    /// - If the sequence of tokens does not contain the 'HALT' instruction.
    /// - If any instruction in the sequence is not a valid predefined token.
    /// - If the number of parameters for any instruction does not match the expected number.
    /// - If the parameter is not in hexadecimal base.
    /// - If there are issues while writing the validated tokens to the output file.
    pub fn validate(&self) -> Result<(), Error> {
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
            ("HALT", 0),
            ("@", 0)    // TODO
        ]);

        if !self.tokens.contains(&"HALT".to_string()) {
            return Err(Error::new(ErrorKind::Other, "Missing 'HALT' instruction"));
        }

        for token in &self.tokens {
            let instruction: Vec<&str> = token.split_whitespace().collect();

            if !valid_tokens.contains_key(&instruction[0]) {
                let msg: String = format!("Invalid instruction '{}'", instruction[0]);
                return Err(Error::new(ErrorKind::Other, msg));
            }

            let params: Vec<&str> = instruction[1..].to_vec();
            let num_params: usize = params.len();

            if &num_params != valid_tokens.get(instruction[0]).unwrap() {
                let msg: String = format!(
                    "Invalid number of parameters in '{}', only has {} but get {} -> {}", 
                    instruction[0], 
                    valid_tokens.get(instruction[0]).unwrap(), 
                    num_params, 
                    token
                );

                return Err(Error::new(ErrorKind::Other, msg));
            }

            if num_params > 0 && !self.valid_params(&params) {
                let msg: String = format!("Invalid parameters in '{token}', the parameters must be in hexadecimal base");
                return Err(Error::new(ErrorKind::Other, msg));
            }
        }
        
        self.write_file()?;
        Ok(())
    }
}