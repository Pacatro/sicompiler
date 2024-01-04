use std::fs;
use std::io;

pub struct Tokenizer {
    input: String
}

impl Tokenizer {
    fn remove_comments(&self, line: &mut String) {
        if line.is_empty() || !line.contains(';') {
            return;
        }

        line.drain(line.find(';').unwrap()..);
        *line = line.trim_end().to_string();
    }
    
    pub fn new(input: String) -> Tokenizer { Tokenizer { input } }
    
    pub fn tokenize(&self) -> Result<Vec<String>, io::Error> {
        let mut tokens: Vec<String> = Vec::new();
        let content: String = fs::read_to_string(&self.input)?;

        if content.is_empty() {
            return Ok(tokens);
        }
        
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