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
    fn is_hex(params: &Vec<String>) -> bool {
        params.iter()
            .next()
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .is_ascii_hexdigit()
    }

    fn write_file(&self) -> Result<(), Error> {
        let mut file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.output_file)?;

        for variable in self.tokens.variables() {
            file.write_all(format!("{} {}\n", variable.dir(), variable.name()).as_bytes())?;
        }

        writeln!(file, "@")?;
        writeln!(file, "{}", self.tokens.init().dir.clone())?;
        writeln!(file, "@")?;

        for instruction in self.tokens.instructions() {
            file.write_all(format!("{} {}\n", instruction.mnemonic(), instruction.clone().params().join(" ")).as_bytes())?;
        }

        Ok(())
    }

    fn validate_program(&self) -> Result<(), Error> {
        if self.tokens.variables().is_empty() || self.tokens.instructions().is_empty() {
            return Err(Error::new(ErrorKind::Other, "Empty program"));
        }

        Ok(())
    }

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

    fn validate_init(&self) -> Result<(), Error> {
        if !Validator::is_hex(&vec![self.tokens.init().dir.clone()]) {
            let msg: String = format!("The init dir must be in hex base '{}'", self.tokens.init().dir);
            return Err(Error::new(ErrorKind::Other, msg));
        }

        Ok(())
    }

    fn validate_instructions(&self) -> Result<(), Error> {
        let valid_instructions: HashMap<&str, Instruction> = HashMap::from([
            ("CRA", Instruction::new("CRA", vec![])),
            ("CTA", Instruction::new("CTA", vec![])),
            ("ITA", Instruction::new("ITA", vec![])),
            ("CRF", Instruction::new("CRF", vec![])),
            ("CTF", Instruction::new("CTF", vec![])),
            ("SFZ", Instruction::new("SFZ", vec![])),
            ("ROR_F_ACC", Instruction::new("ROR_F_ACC", vec![])),
            ("ROL_F_ACC", Instruction::new("ROL_F_ACC", vec![])),
            ("ADD", Instruction::new("ADD", vec!["0x1234"])),
            ("ADDI", Instruction::new("ADDI", vec!["0x1234"])),
            ("STA", Instruction::new("STA", vec!["0x1234"])),
            ("JMP", Instruction::new("JMP", vec!["0x1234"])),
            ("JMPI", Instruction::new("JMPI", vec!["0x1234"])),
            ("HALT", Instruction::new("HALT", vec![]))
        ]);

        if !self.tokens.instructions().contains(valid_instructions.get("HALT").unwrap()) {
            return Err(Error::new(ErrorKind::Other, "Missing 'HALT' instruction"));
        }

        for instruction in self.tokens.instructions() {
            if !valid_instructions.contains_key(instruction.mnemonic()) {
                let msg: String = format!("Invalid instruction '{}'", instruction.mnemonic());
                return Err(Error::new(ErrorKind::Other, msg));
            }
            
            let params: Vec<String> = instruction.clone().params();
            let num_params: usize = instruction.clone().params().len();

            if instruction.clone().flag() != valid_instructions.get(instruction.mnemonic()).unwrap().clone().flag() {
                let msg: String = format!(
                    "The instruction '{}' must have at least one parameter", 
                    instruction.mnemonic(), 
                );

                return Err(Error::new(ErrorKind::Other, msg));
            }

            if num_params != valid_instructions.get(instruction.mnemonic()).unwrap().clone().params().len() {
                let msg: String = format!(
                    "Invalid number of parameters in '{}', only has {} but get {}", 
                    instruction.mnemonic(), 
                    valid_instructions.get(instruction.mnemonic()).unwrap().clone().params().len(), 
                    num_params
                );

                return Err(Error::new(ErrorKind::Other, msg));
            }
        
            if num_params > 0 && !Validator::is_hex(&params) {
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

    pub fn validate(&self) -> Result<(), Error> {
        self.validate_program()?;
        self.validate_variables()?;
        self.validate_init()?;
        self.validate_instructions()?;
        self.write_file()?;
        
        Ok(())
    }
}