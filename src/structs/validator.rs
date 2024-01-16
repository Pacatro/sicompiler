use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, Error, ErrorKind};

use crate::models::{program::Program, instruction::Instruction};

/// The `Validator` struct is responsible for validating a sequence of tokens
/// representing a custom assembly language. It ensures that each instruction
/// is valid and adheres to the expected format, raising errors if any issues are detected.
pub struct Validator {
    output_file: String,
    tokens: Program
}

impl Validator {
    /// Checks if all strings in the given vector represent valid hexadecimal values.
    /// 
    /// ## Arguments
    /// 
    /// - `params` - A reference to a vector of strings to be checked.
    /// 
    /// ## Returns
    /// 
    /// - `bool` - Returns `true` if all strings in the vector are valid hexadecimal values, otherwise `false`.
    /// 
    /// ## Example
    /// 
    /// ```
    /// let params = vec!["1a", "2F", "4D"];
    /// let result = is_hex(&params);
    /// assert_eq!(result, true);
    /// ```
    /// 
    fn is_hex(params: &Vec<String>) -> bool {
        for param in params {
            if !param.chars().next().unwrap().is_ascii_hexdigit() {
                return false;
            }
        }

        true
    }

    /// Writes the tokenized information to an output file.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), Error>` - Result indicating success or an `Error` if any issues occur during file writing.
    ///     
    fn write_file(&self) -> Result<(), Error> {
        let mut file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.output_file)?;

        for variable in self.tokens.variables() {
            file.write_all(format!("{} {}\n", variable.dir(), variable.name()).as_bytes())?;
        }

        writeln!(file, "@")?;
        writeln!(file, "{}", self.tokens.init().dir())?;
        writeln!(file, "@")?;

        for instruction in self.tokens.instructions() {
            file.write_all(format!("{} {}\n", instruction.mnemonic(), instruction.clone().params().join(" ")).as_bytes())?;
        }

        Ok(())
    }

    /// Validates the tokenized program to ensure it contains both instructions and variables sections.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), Error>` - Result indicating success or an `Error` if there is no instructions or variables section.
    /// 
    fn validate_program(&self) -> Result<(), Error> {
        if self.tokens.variables().is_empty() || self.tokens.instructions().is_empty() {
            return Err(Error::new(ErrorKind::Other, "There is not any instructions or variables section"));
        }

        Ok(())
    }

    /// Validates the tokenized variables to ensure both directory and name are in hexadecimal format.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), Error>` - Result indicating success or an `Error` if any variable has a non-hexadecimal directory or name.
    /// 
    fn validate_variables(&self) -> Result<(), Error> {
        for variable in self.tokens.variables() {
            let var: Vec<String> = vec![variable.dir().to_string(), variable.name().to_string()];
            
            if !Validator::is_hex(&var) {
                let msg: String = format!("The varibale dir and name must be in hex base '{} {}'", variable.dir(), variable.name());
                return Err(Error::new(ErrorKind::Other, msg));
            } 
        }

        Ok(())
    }

    /// Validates the tokenized initialization directory to ensure it is in hexadecimal format.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), Error>` - Result indicating success or an `Error` if the initialization directory is not in hexadecimal format.
    /// 
    fn validate_init(&self) -> Result<(), Error> {
        if !Validator::is_hex(&vec![self.tokens.init().dir().to_string()]) {
            let msg: String = format!("The init dir must be in hex base '{}'", self.tokens.init().dir());
            return Err(Error::new(ErrorKind::Other, msg));
        }

        Ok(())
    }

    /// Validates the tokenized instructions to ensure they are valid and have the correct parameters.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), Error>` - Result indicating success or an `Error` if any instruction is invalid or has incorrect parameters.
    /// 
    fn validate_instructions(&self, repertoire: &HashMap<String, Instruction>) -> Result<(), Error> {
        for instruction in self.tokens.instructions() {
            if !repertoire.contains_key(instruction.mnemonic()) {
                let msg: String = format!("Invalid instruction, '{}' does not appear in the repertoire", instruction.mnemonic());
                return Err(Error::new(ErrorKind::Other, msg));
            }
            
            let rep_instruction: &Instruction = match repertoire.get(instruction.mnemonic()) {
                Some(instruction) => instruction,
                None => {
                    return Err(Error::new(ErrorKind::Other, 
                        format!("The instruction '{}' is not defined in the repertoire", instruction.mnemonic())))
                }
            };
            
            if instruction.flag() != rep_instruction.flag() {
                let msg: String = format!(
                    "The instruction '{}' must have some parameters", 
                    instruction.mnemonic(),
                );
                
                return Err(Error::new(ErrorKind::Other, msg));
            }
            
            /*
            TODO: Figure out how many params can have each instruction

            !At this momment, I don't have any idea to know how many params can have an instructions based on the repertoire.
            
            let num_params: usize = instruction.params().len();
            
            if num_params != repertoire.get(instruction.mnemonic()).unwrap().params().len() {
                let msg: String = format!(
                    "Invalid number of parameters in '{}', only has {} but get {}", 
                    instruction.mnemonic(), 
                    repertoire.get(instruction.mnemonic()).unwrap().params().len(), 
                    num_params
                );

                return Err(Error::new(ErrorKind::Other, msg));
            } 

            */
        
            let params: &Vec<String> = instruction.params();

            if instruction.flag() && !Validator::is_hex(&params) {
                let msg: String = format!("Invalid parameters in '{}', the parameters must be in hex base", instruction.mnemonic());
                return Err(Error::new(ErrorKind::Other, msg));
            }
        }

        Ok(())
    }
    
    /// Creates a new `Validator` instance with the specified tokens and output file.
    pub fn new(tokens: Program, output_file: String) -> Validator {
        Validator { tokens, output_file }
    }

    /// Validates the tokenized program, variables, initialization directory, and instructions,
    /// and writes the validated information to an output file.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), Error>` - Result indicating success or an `Error` if any validation step fails.
    /// 
    pub fn validate(&self, repertoire: &HashMap<String, Instruction>) -> Result<(), Error> {
        self.validate_program()?;
        self.validate_variables()?;
        self.validate_init()?;
        self.validate_instructions(repertoire)?;
        self.write_file()?;
        
        Ok(())
    }
}