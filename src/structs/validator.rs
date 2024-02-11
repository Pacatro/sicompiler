use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

use crate::models::{program::Program, instruction::Instruction};
use crate::errors::error::SicompilerError;

/// The `Validator` struct is responsible for validating a sequence of tokens
/// representing a custom assembly language. It ensures that each instruction
/// is valid and adheres to the expected format, raising errors if any issues are detected.
pub struct Validator {
    output_file: String,
    tokens: Program
}

impl Validator {
    fn is_hex(params: &Vec<String>) -> bool {
        params.iter().all(|param: &String| {
            param.chars()
                .next()
                .map_or(false, |first_char: char| first_char.is_ascii_hexdigit())
        })
    }

    /// Writes the tokenized information to an output file.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<(), SicompilerError>` - Result indicating success or an `Error` if any issues occur during file writing.
    ///     
    fn write_file(&self) -> Result<(), SicompilerError> {
        let mut file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.output_file)
            .map_err(|e: io::Error| 
                SicompilerError::Io(io::Error::new(e.kind(), format!("Can't open {}", self.output_file)))
            )?;

        for variable in self.tokens.variables() {
            file.write_all(format!("{} {}\n", variable.dir(), variable.name()).as_bytes())?;
        }

        writeln!(file, "@")?;
        writeln!(file, "{}", self.tokens.init().dir())?;
        writeln!(file, "@")?;

        for instruction in self.tokens.instructions() {
            file.write_all(format!("{} {}\n", instruction.mnemonic(), instruction.params().join(" ")).as_bytes())?;
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
    /// - `Result<(), SicompilerError>` - Result indicating success or an `Error` if there is no instructions or variables section.
    /// 
    fn validate_program(&self) -> Result<(), SicompilerError> {
        if self.tokens.variables().is_empty() || self.tokens.instructions().is_empty() {
            return Err(SicompilerError::ValidationError("There is not any instructions or variables section".to_string()));
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
    /// - `Result<(), SicompilerError>` - Result indicating success or an `Error` if any variable has a non-hexadecimal directory or name.
    /// 
    fn validate_variables(&self) -> Result<(), SicompilerError> {
        for variable in self.tokens.variables() {
            let var: Vec<String> = vec![variable.dir().to_string(), variable.name().to_string()];
            
            if !Validator::is_hex(&var) {
                return Err(SicompilerError::ValidationError(
                    format!("The varibale dir and name must be in hex base '{} {}'", variable.dir(), variable.name())
                ));
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
    /// - `Result<(), SicompilerError>` - Result indicating success or an `Error` if the initialization directory is not in hexadecimal format.
    /// 
    fn validate_init(&self) -> Result<(), SicompilerError> {
        if !Validator::is_hex(&vec![self.tokens.init().dir().to_string()]) {
            return Err(SicompilerError::ValidationError(
                format!("The init dir must be in hex base '{}'", self.tokens.init().dir())
            ));
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
    /// - `Result<(), SicompilerError>` - Result indicating success or an `Error` if any instruction is invalid or has incorrect parameters.
    /// 
    fn validate_instructions(&self, repertoire: &HashMap<String, Instruction>) -> Result<(), SicompilerError> {
        for instruction in self.tokens.instructions() {
            if !repertoire.contains_key(instruction.mnemonic()) {
                return Err(SicompilerError::ValidationError(
                    format!("Invalid instruction, '{}' does not appear in the repertoire", instruction.mnemonic())
                ));
            }
            
            let rep_instruction: &Instruction = match repertoire.get(instruction.mnemonic()) {
                Some(instruction) => instruction,
                None => {
                    return Err(SicompilerError::ValidationError(
                        format!("The instruction '{}' is not defined in the repertoire", instruction.mnemonic())
                    ));
                }
            };
            
            if instruction.flag() != rep_instruction.flag() {                
                return Err(SicompilerError::ValidationError(
                    format!("The instruction '{}' must have some parameters", instruction.mnemonic())
                ));
            }

            let params: &Vec<String> = instruction.params();
            let rep_params: &Vec<String> = rep_instruction.params();
            
            if params.len() != rep_params.len() {
                return Err(SicompilerError::ValidationError(
                    format!(
                        "Invalid number of parameters in '{}', only has {} but get {}", 
                        instruction.mnemonic(), 
                        rep_params.len(), 
                        instruction.params().len()
                    )
                ));
            } 

            if instruction.flag() && !Validator::is_hex(&params) {
                return Err(SicompilerError::ValidationError(
                    format!("Invalid parameters in '{}', the parameters must be in hex base", instruction.mnemonic())
                ));
            }
        }

        Ok(())
    }
    
    /// Creates a new `Validator` instance with the specified tokens and output file.
    pub fn new(tokens: Program, output_file: &str) -> Validator {
        Validator { tokens, output_file: output_file.to_string() }
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
    /// - `Result<(), SicompilerError>` - Result indicating success or an `Error` if any validation step fails.
    /// 
    pub fn validate(&self, repertoire: &HashMap<String, Instruction>) -> Result<(), SicompilerError> {
        self.validate_program()?;
        self.validate_variables()?;
        self.validate_init()?;
        self.validate_instructions(repertoire)?;
        self.write_file()?;
        
        Ok(())
    }
}