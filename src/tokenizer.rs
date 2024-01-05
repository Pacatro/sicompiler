use std::fs;
use std::io;

pub struct Tokenizer {
    input: String
}

/// The `Tokenizer` struct is responsible for tokenizing input source code,
/// removing comments and empty lines, and providing a sequence of valid code lines.
impl Tokenizer {
    fn remove_comments(&self, line: &mut String) {
        if line.is_empty() || !line.contains(';') { return }
        line.drain(line.find(';').unwrap()..);
        *line = line.trim_end().to_string();
    }
    
    /// Creates a new `Tokenizer` instance with the specified input file name.
    ///
    /// # Arguments
    ///
    /// - `input` - The name of the input file to be tokenized.
    pub fn new(input: String) -> Tokenizer { Tokenizer { input } }
    
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
    pub fn tokenize(&self) -> Result<Vec<String>, io::Error> {
        let mut tokens: Vec<String> = Vec::new();
        let content: String = fs::read_to_string(&self.input)?;

        if content.is_empty() { return Ok(tokens) }
        
        for line in content.lines() {
            let mut string_line: String = line.to_string();
            self.remove_comments(&mut string_line);

            if !string_line.is_empty() {
                tokens.push(string_line);
            }
        }

        Ok(tokens)
    }
}