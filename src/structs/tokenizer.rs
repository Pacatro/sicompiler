use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};

use crate::models::{instruction::Instruction, variable::Variable, init::Init, program::Program};

/// The `Tokenizer` struct is responsible for tokenizing input source code,
/// removing comments and empty lines, and providing a sequence of valid code lines.
pub struct Tokenizer {
    input: String,
    rep: String
}

impl Tokenizer {
    /// Removes one-line comments and trims leading and trailing whitespace from each line in the input content.
    ///
    /// # Arguments
    ///
    /// - `content` - A string containing the content with one-line comments.
    ///
    /// # Returns
    ///
    /// A new string with one-line comments removed and leading/trailing whitespace trimmed from each line.
    ///
    /// # Examples
    ///
    /// ```
    /// let content = "This is a line without comments\n
    ///                This is a line with a comment ;This is a one-line comment\n
    ///                Another line without comments";
    ///
    /// let result = remove_oneline_comments(content);
    /// assert_eq!(result, "This is a line without comments;\nThis is a line with a comment\nAnother line without comments;");
    /// ```
    ///
    fn remove_oneline_comments(content: &str) -> String {
        content.lines()
            .map(|line| {
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

    /// Removes multiline comments enclosed by '***' from the input content.
    ///
    /// # Arguments
    ///
    /// - `content` - A string containing the content with multiline comments.
    ///
    /// # Returns
    ///
    /// A new string with multiline comments removed.
    ///
    /// # Examples
    ///
    /// ```
    /// let content = "This is a line without comments.\n
    ///                *** This 
    ///                 is a multiline 
    ///                 comment ***\n
    ///                Another line without comments.";
    ///
    /// let result = remove_multiline_comments(content);
    /// assert_eq!(result, "This is a line without comments.\nAnother line without comments.\n");
    /// ```
    ///
    /// # Note
    ///
    /// - The function considers lines starting with '***' as the beginning of a comment block.
    /// - It also considers lines ending with '***' as the end of a comment block.
    /// - Lines within the comment block are ignored and not included in the result.
    /// - The function preserves non-comment lines in the result.
    ///
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
    /// A vector of `Varibles` instances or an Error.
    /// 
    fn tokenize_variables(section: &str) -> Result<Vec<Variable>, Error> {
        let mut variables: Vec<Variable> = Vec::new();
        
        for token in section.lines() {
            if token.is_empty() { continue }
    
            let parts: Vec<&str> = token.split_whitespace().collect();
            
            if parts.len() != 2 { 
                return Err(Error::new(ErrorKind::InvalidData, "Invalid variable format, the correct way is <DIR NAME>"));
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
    fn tokenize_init(section: &str) -> Result<Init, Error> {
        if section.is_empty() { 
            return Err(Error::new(ErrorKind::InvalidData, "There is no any Init section."));
        }

        let valid_section: Vec<&str> = section.split_whitespace().collect();

        if valid_section.is_empty() { 
            return Err(Error::new(ErrorKind::InvalidData, "There is no any Init address."));
        }

        if valid_section.len() > 1 {
            return Err(Error::new(ErrorKind::InvalidData, "There is more than one Init address."));
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
    pub fn new(input: String, rep: String) -> Tokenizer { 
        Tokenizer { input, rep }
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
    pub fn tokenize_repertoire(&self) -> Result<HashMap<String, Instruction>, Error> {
        let mut repertorie: HashMap<String, Instruction> = HashMap::new();
        
        let content: String = fs::read_to_string(&self.rep)?;

        if !content.contains("$") {
            let msg = format!("Invalid repertoire structure, the file must contain a microprogram section.");
            return Err(Error::new(ErrorKind::Other, msg));
        }

        let content: Vec<&str> = content.split('$').collect();

        let mut instructions_part: &str = content[2];

        if instructions_part.starts_with("\n") {
            instructions_part = &instructions_part[1..];
        }

        if instructions_part.lines().count() > 32 {
            let msg: String = format!("Invalid number of instructions, the max is 32 but get {}", instructions_part.lines().count());
            return Err(Error::new(ErrorKind::Other, msg));
        }

        for token in instructions_part.lines() {
            if token.is_empty() { continue }
            
            let parts: Vec<&str> = token.split_whitespace().collect();

            let mnemonic: String = parts[0].to_string();
            let flag: bool = parts[1] == "true";

            let mut instruction: Instruction = Instruction::new(&mnemonic, vec![]);

            if flag {
                instruction.set_flag(true);
                
                //* An instruction only have 1 argument but  
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
    pub fn tokenize(&self) -> Result<Program, Error> {
        let mut content: String = fs::read_to_string(&self.input)?;
        
        if content.is_empty() { 
            return Err(Error::new(ErrorKind::Other, "The file is empty"));
        }

        content = Tokenizer::remove_oneline_comments(&content);
        content = Tokenizer::remove_multiline_comments(&content);

        let sections: Vec<&str> = content.split('@').collect();

        if sections.len() != 3 {
            return Err(Error::new(ErrorKind::Other, 
                       format!("Invalid number of sections, must be 3 but get {}", sections.len())));
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
            return Err(Error::new(ErrorKind::Other, "No init dir found"));
        }
        
        let mut instructions: Vec<Instruction> = Vec::new();
        if let Some(instruction_section) = sections.get(2) {
            instructions = Tokenizer::tokenize_instructions(instruction_section);
        }

        Ok(Program::new(variables, init, instructions))
    }
}