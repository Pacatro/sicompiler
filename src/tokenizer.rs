use std::fs;
use std::io::{Error, ErrorKind};

use crate::models::{instruction::Instruction, variable::Variable, init::Init, program::Program};

/// The `Tokenizer` struct is responsible for tokenizing input source code,
/// removing comments and empty lines, and providing a sequence of valid code lines.
pub struct Tokenizer {
    input: String
}

impl Tokenizer {
    /// Removes comments from each line of the input content.
    /// ## Returns 
    /// A string with comments removed.
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

    /// Tokenizes instruction 
    /// 
    /// ## Arguments
    /// - `section` - The instruction section of the file
    /// 
    /// ## Returns 
    /// A vector of `Instruction` instances.
    fn tokenize_instructions(section: &str) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
            
            let parts: Vec<&str> = token.split_whitespace().collect();

            instructions.push(Instruction::new(parts[0], parts[1..].to_vec()));
        }

        instructions
    }

    /// Tokenizes varibles 
    /// 
    /// ## Arguments
    /// - `section` - The varibles section of the file
    /// 
    /// ## Returns 
    /// A vector of `Varibles` instances.
    fn tokenize_variables(section: &str) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
            let parts: Vec<&str> = token.split_whitespace().collect();
            variables.push(Variable::new(parts[0], parts[1]));
        }

        variables
    }

    /// Tokenizes init section 
    /// 
    /// ## Arguments
    /// - `section` - The init section of the file
    /// 
    /// ## Returns 
    /// A `Init` instances.
    fn tokenize_init(section: &str) -> Init {
        if section.is_empty() { 
            return Init { dir: String::from("")}
        
        }

        let valid_section: Vec<&str> = section.split_whitespace().collect();

        if valid_section.is_empty() { 
            return Init { dir: String::from("")}
        }

        let dir: String = valid_section[0].to_string();
        Init { dir }
    }
    
    /// Creates a new `Tokenizer` instance with the specified input file name.
    ///
    /// ## Arguments
    ///
    /// - `input` - The name of the input file to be tokenized.
    pub fn new(input: String) -> Tokenizer { 
        Tokenizer { input }
    }
    
    /// Tokenizes the content of the input file and returns a `Result` containing a `Program` or an `Error`.
    /// 
    /// ## Arguments
    /// 
    /// - `&self` - Reference to the `Tokenizer` instance.
    /// 
    /// ## Returns
    /// 
    /// - `Result<Program, Error>` - Result containing a `Program` instance if successful, or an `Error` if any issues occur.
    /// 
    /// ## Errors
    /// 
    /// Returns an `Error` if:
    /// 
    /// - The file is empty.
    /// - The number of sections in the file is not equal to 3.
    /// - No init dir is found.
    /// 
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
            return Err(Error::new(ErrorKind::Other, "No init dir found"));
        }
        
        let mut instructions: Vec<Instruction> = Vec::new();
        if let Some(instruction_section) = sections.get(2) {
            instructions = Tokenizer::tokenize_instructions(instruction_section);
        }

        Ok(Program::new(variables, init, instructions))
    }
}