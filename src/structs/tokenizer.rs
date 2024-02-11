use std::collections::HashMap;
use std::{fs, io};

use crate::models::{instruction::Instruction, variable::Variable, init::Init, program::Program};
use crate::errors::error::SicompilerError;

/// The `Tokenizer` struct is responsible for tokenizing input source code,
/// removing comments and empty lines, and providing a sequence of valid code lines.
pub struct Tokenizer {
    input: String,
    rep: String
}

impl Tokenizer {
    fn remove_oneline_comments(content: &str) -> String {
        content.lines()
            .map(|line: &str| {
                let trimmed_line: &str = line.trim();
                if trimmed_line.is_empty() {
                    String::from(line)
                } else if let Some(index) = trimmed_line.find(';') {
                    String::from(&trimmed_line[..index])
                } else {
                    String::from(trimmed_line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn remove_multiline_comments(content: &str) -> String {
        let mut result: String = String::new();
        let mut in_comment: bool = false;

        for line in content.lines() {
            let trimmed_line: &str = line.trim();

            if trimmed_line.starts_with("***") {
                in_comment = true;
                continue;
            }

            if trimmed_line.ends_with("***") {
                in_comment = false;
                continue;
            }

            if !in_comment {
                result.push_str(line);
                result.push('\n');
            }
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
    /// 
    fn tokenize_instructions(section: &str) -> Vec<Instruction> {
        section
            .lines()
            .filter(|token| !token.is_empty())
            .map(|token| {
                let parts: Vec<&str> = token.split_whitespace().collect();
                Instruction::new(parts[0], parts[1..].to_vec())
            })
            .collect()
    }

    /// Tokenizes varibles 
    /// 
    /// ## Arguments
    /// - `section` - The varibles section of the file
    /// 
    /// ## Returns 
    /// A vector of `Varibles` instances or an Error.
    /// 
    fn tokenize_variables(section: &str) -> Result<Vec<Variable>, SicompilerError> {
        let mut variables: Vec<Variable> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
    
            let parts: Vec<&str> = token.split_whitespace().collect();
            
            if parts.len() != 2 { 
                return Err(
                    SicompilerError::TokenizationError(format!("Invalid variable format, the correct way is <DIR NAME>"))
                );
            }

            variables.push(Variable::new(parts[0], parts[1]));
        }

        Ok(variables)
    }

    /// Tokenizes init section 
    /// 
    /// ## Arguments
    /// - `section` - The init section of the file
    /// 
    /// ## Returns 
    /// A `Init` instances or an Error.
    /// 
    fn tokenize_init(section: &str) -> Result<Init, SicompilerError> {
        if section.is_empty() { 
            return Err(SicompilerError::TokenizationError(format!("There is no any Init section.")));
        }

        let valid_section: Vec<&str> = section.split_whitespace().collect();

        if valid_section.is_empty() { 
            return Err(SicompilerError::TokenizationError(format!("There is no any Init address.")));
        }

        if valid_section.len() > 1 {
            return Err(SicompilerError::TokenizationError(format!("There is more than one Init address.")));
        }

        let dir: &str = valid_section[0];
        Ok(Init::new(dir))
    }
    
    /// Creates a new `Tokenizer` instance with the specified input file name.
    ///
    /// ## Arguments
    ///
    /// - `input` - The name of the input file to be tokenized.
    /// 
    pub fn new(input: &str, rep: &str) -> Tokenizer { 
        Tokenizer { input: input.to_string(), rep: rep.to_string() }
    }

    /// Tokenizes the content of a repertoire file, creating a mapping of mnemonics to instructions.
    ///
    /// # Arguments
    ///
    /// - `repertoire_input` - A string representing the path to the repertoire file.
    ///
    /// # Returns
    ///
    /// - `Result<HashMap<String, Instruction>, Error>` - Result containing a mapping of mnemonics to instructions
    ///   if successful, or an `Error` if any issues occur during tokenization or file reading.
    ///
    pub fn tokenize_repertoire(&self) -> Result<HashMap<String, Instruction>, SicompilerError> {
        let mut repertorie: HashMap<String, Instruction> = HashMap::new();
        
        let content: String = fs::read_to_string(&self.rep)?;

        if !content.contains("$") {
            return Err(SicompilerError::TokenizationError(
                format!("Invalid repertoire structure, the file must contain a microprogram section.")
            ));
        }

        let content: Vec<&str> = content.split('$').collect();

        let mut instructions_part: &str = content[2];

        if instructions_part.starts_with("\n") {
            instructions_part = &instructions_part[1..];
        }

        if instructions_part.lines().count() > 32 {
            return Err(SicompilerError::TokenizationError(
                format!("Invalid number of instructions, the max is 32 but get {}", instructions_part.lines().count())
            ));
        }

        for token in instructions_part.lines() {
            if token.is_empty() { continue }
            
            let parts: Vec<&str> = token.split_whitespace().collect();

            let mnemonic: String = parts[0].to_string();
            let flag: bool = parts[1] == "true";

            let mut instruction: Instruction = Instruction::new(&mnemonic, vec![]);

            if flag {
                instruction.set_flag(true);
                
                //* An instruction only have 1 argument  
                instruction.set_params(vec!["0x123"]);
            }

            repertorie.insert(mnemonic, instruction);
        }
        
        Ok(repertorie)
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
    pub fn tokenize(&self) -> Result<Program, SicompilerError> {
        let mut content: String = fs::read_to_string(&self.input)
            .map_err(|e: io::Error| 
                SicompilerError::Io(io::Error::new(e.kind(), format!("Can't open {}", self.input)))
            )?;
        
        if content.is_empty() { 
            return Err(SicompilerError::TokenizationError(format!("The file is empty")));
        }

        content = Tokenizer::remove_oneline_comments(&content);
        content = Tokenizer::remove_multiline_comments(&content);

        let sections: Vec<&str> = content.split('@').collect();

        if sections.len() != 3 {
            return Err(SicompilerError::TokenizationError(
                format!("Invalid number of sections, must be 3 but get {}", sections.len())
            ))
        }
        
        let mut variables: Vec<Variable> = Vec::new();
        if let Some(variable_section) = sections.get(0) {
            variables = Tokenizer::tokenize_variables(variable_section)?;
        }
        
        let mut init: Init = Init::new("");
        if let Some(init_section) = sections.get(1) {
            init = Tokenizer::tokenize_init(init_section)?;
        }

        if init.dir().is_empty() {
            return Err(SicompilerError::TokenizationError(
                format!("No init dir found")
            ));
        }
        
        let mut instructions: Vec<Instruction> = Vec::new();
        if let Some(instruction_section) = sections.get(2) {
            instructions = Tokenizer::tokenize_instructions(instruction_section);
        }

        Ok(Program::new(variables, init, instructions))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::models::{
        instruction::Instruction, 
        variable::Variable, 
        init::Init, 
        program::Program
    };

    use crate::errors::error::SicompilerError;

    use super::Tokenizer;

    #[test]
    fn test_remove_one_line_comment() {
        let content: &str = "This is a line without comments
This is a line with a comment ;This is a one-line comment
Another line without comments";

        let result: String = Tokenizer::remove_oneline_comments(content);
        assert_eq!(result, "This is a line without comments\nThis is a line with a comment \nAnother line without comments");
    }

    #[test]
    fn test_remove_multi_line_comment() {
        let content = "This is a line without comments.
*** This 
is a multiline 
comment ***
Another line without comments.";
    
        let result: String = Tokenizer::remove_multiline_comments(content);
        assert_eq!(result, "This is a line without comments.\nAnother line without comments.\n");
    }

    #[test]
    fn test_tokenize_instructions() {
        let section: &str = "HALT\nADD 1";
        let instructions: Vec<Instruction> = Tokenizer::tokenize_instructions(section);

        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].mnemonic(), "HALT");
        assert_eq!(instructions[0].params().len(), 0);
        assert_eq!(instructions[1].mnemonic(), "ADD");
        assert_eq!(instructions[1].params().len(), 1);
    }

    #[test]
    fn test_tokenize_variables() -> Result<(), SicompilerError> {
        let section: &str = "1 0003\n3 0000";

        let variables: Vec<Variable> = Tokenizer::tokenize_variables(section)?;

        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0].dir(), "1");
        assert_eq!(variables[0].name(), "0003");
        assert_eq!(variables[1].dir(), "3");
        assert_eq!(variables[1].name(), "0000");

        Ok(())
    }

    #[test]
    fn test_tokenize_init() -> Result<(), SicompilerError> {
        let section: &str = "1";
        let init: Init = Tokenizer::tokenize_init(section)?;

        assert_eq!(init.dir(), "1");

        Ok(())
    }

    #[test]
    fn test_tokenize() -> Result<(), SicompilerError> {
        let tokenizer: Tokenizer = Tokenizer::new("tests-files/test-input.txt", "tests-files/test-repertoire.rep");
        let program: Program = tokenizer.tokenize()?;

        assert_eq!(program.variables().len(), 3);
        assert_eq!(program.init().dir(), "6");
        assert_eq!(program.instructions().len(), 2);

        Ok(())
    }

    #[test]
    fn test_tokenize_repertoire() -> Result<(), SicompilerError> {
        let tokenizer: Tokenizer = Tokenizer::new("tests-files/test-input.txt", "tests-files/test-repertoire.rep");
        let repertoire: HashMap<String, Instruction> = tokenizer.tokenize_repertoire()?;

        assert_eq!(repertoire.len(), 2);
        assert_eq!(repertoire.get("HALT").unwrap().mnemonic(), "HALT");
        assert_eq!(repertoire.get("HALT").unwrap().params().len(), 0);
        assert_eq!(repertoire.get("ADD").unwrap().mnemonic(), "ADD");
        assert_eq!(repertoire.get("ADD").unwrap().params().len(), 1);
        Ok(())
    }

    #[test]
    fn test_tokenize_empty_file() {
        let tokenizer: Tokenizer = Tokenizer::new("tests-files/fails-files/empty-file.txt", "tests-files/test-repertoire.rep");
        let result: Result<Program, SicompilerError> = tokenizer.tokenize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Tokenization error: The file is empty");
    }

    #[test]
    fn test_tokenize_invalid_file() {
        let tokenizer: Tokenizer = Tokenizer::new("tests-files/fails-files/invalid-file.txt", "tests-files/test-repertoire.rep");
        let result: Result<Program, SicompilerError> = tokenizer.tokenize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Tokenization error: Invalid number of sections, must be 3 but get 1");
    }

    #[test]
    fn test_tokenize_invalid_repertoire() {
        let tokenizer: Tokenizer = Tokenizer::new("tests-files/test-input.txt", "tests-files/fails-files/invalid-repertoire.rep");
        let result: Result<HashMap<String, Instruction>, SicompilerError> = tokenizer.tokenize_repertoire();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Tokenization error: Invalid repertoire structure, the file must contain a microprogram section.");
    }
}
