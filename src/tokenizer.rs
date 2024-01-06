use std::fs;
use std::io::{Error, ErrorKind};

use crate::models::{instruction::Instruction, variable::Variable, init::Init, program::Program};

/// The `Tokenizer` struct is responsible for tokenizing input source code,
/// removing comments and empty lines, and providing a sequence of valid code lines.
pub struct Tokenizer {
    input: String
}

impl Tokenizer {
    fn remove_comments(content: &str) -> String {
        let mut result = String::new();

        for line in content.lines() {
            let mut current_line: String = line.to_string();

            if current_line.is_empty() || !current_line.contains(';') {
                result.push_str(&current_line);
            } else {
                current_line.truncate(current_line.find(';').unwrap_or_else(|| current_line.len()));
                result.push_str(&current_line);
            }

            result.push('\n');
        }

        result
    }

    fn tokenize_instructions(section: &str) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
            
            let parts: Vec<&str> = token.split_whitespace().collect();

            instructions.push(Instruction::new(parts[0], parts[1..].to_vec()));
        }

        instructions
    }

    fn tokenize_variables(section: &str) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
            let parts: Vec<&str> = token.split_whitespace().collect();
            variables.push(Variable::new(parts[0], parts[1]));
        }

        variables
    }

    fn tokenize_init(section: &str) -> Init {
        if section.is_empty() { return Init { dir: String::from("")}}
        let dir: String = section.split_whitespace().collect::<Vec<&str>>()[0].to_string();
        Init { dir }
    }
    
    /// Creates a new `Tokenizer` instance with the specified input file name.
    ///
    /// # Arguments
    ///
    /// - `input` - The name of the input file to be tokenized.
    pub fn new(input: String) -> Tokenizer { 
        Tokenizer { input }
    }
    
    pub fn tokenize(&self) -> Result<Program, Error> {
        let mut content: String = fs::read_to_string(&self.input)?;
        
        if content.is_empty() { 
            return Err(Error::new(ErrorKind::Other, "The file is empty"));
        }

        content = Tokenizer::remove_comments(&content);

        let sections: Vec<&str> = content.split('@').collect();

        if sections.len() != 3 {
            return Err(Error::new(ErrorKind::Other, "Invalid number of sections"));
        }
        
        let mut variables: Vec<Variable> = Vec::new();
        if let Some(variable_section) = sections.get(0) {
            variables = Tokenizer::tokenize_variables(variable_section);
        }
        
        let mut init: Init = Init { dir: "".to_string() };
        if let Some(init_section) = sections.get(1) {
            init = Tokenizer::tokenize_init(init_section);
        }

        if init.dir.is_empty() {
            return Err(Error::new(ErrorKind::Other, "No init section found"));
        }
        
        let mut instructions: Vec<Instruction> = Vec::new();
        if let Some(instruction_section) = sections.get(2) {
            instructions = Tokenizer::tokenize_instructions(instruction_section);
        }

        Ok(Program::new(variables, init, instructions))
    }
}