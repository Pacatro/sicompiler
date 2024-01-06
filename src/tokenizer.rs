use std::fs;
use std::io::{Error, ErrorKind};

use crate::models::{instruction::Instruction, variable::Variable, init::Init};

/// The `Tokenizer` struct is responsible for tokenizing input source code,
/// removing comments and empty lines, and providing a sequence of valid code lines.
pub struct Tokenizer {
    input: String
}

impl Tokenizer {
    fn remove_comments(&self, content: &String) -> String {
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

    fn tokenize_instructions(&self, section: &str) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
            
            let parts: Vec<String> = token.split_whitespace()
                .map(|s| s.to_string())
                .collect();
            instructions.push(Instruction::new(parts[0].to_string(), parts[1..].to_vec()));
        }

        instructions

    }

    fn tokenize_variables(&self, section: &str) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
        
        for token in section.lines() {
            let parts: Vec<&str> = token.split_whitespace().collect();
            variables.push(Variable::new(parts[0], parts[1]));
        }

        variables
    }

    fn tokenize_init(&self, section: &str) -> Init {
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
    
    /// Tokenizes the content of the input file, removing comments and empty lines.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing a vector of strings representing the tokens from the input file.
    /// If successful, the vector contains the non-empty lines after removing comments.
    /// If the input file is empty, an empty vector is returned.
    /// 
    /// # Errors
    /// 
    /// The function may return an error if there are issues reading the input file.
    pub fn tokenize(&self) -> Result<(Vec<Variable>, Init, Vec<Instruction>), Error> {
        let mut content: String = fs::read_to_string(&self.input)?;
        
        if content.is_empty() { 
            return Err(Error::new(ErrorKind::Other, "The file is empty"));
        }

        content = self.remove_comments(&content);

        let sections: Vec<&str> = content.split('@').collect();
        
        let mut variables: Vec<Variable> = Vec::new();
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut init: Init = Init { dir: "".to_string() };

        if let Some(variable_section) = sections.get(0) {
            variables = self.tokenize_variables(variable_section);
        }

        if let Some(init_section) = sections.get(1) {
            init = self.tokenize_init(init_section);
        }

        if let Some(instruction_section) = sections.get(2) {
            instructions = self.tokenize_instructions(instruction_section);
        }

        let tokens: (Vec<Variable>, Init, Vec<Instruction>) = (variables, init, instructions);
        Ok(tokens)
    }
}