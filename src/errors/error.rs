use std::io;
use std::fmt;

/// Custom error type, encompassing different error scenarios.
#[derive(Debug)]
pub enum SicompilerError {
    /// Represents I/O errors
    Io(io::Error),
    /// Represents errors during the tokenization process.
    TokenizationError(String),
    /// Represents validation errors.
    ValidationError(String)
}

impl fmt::Display for SicompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SicompilerError::Io(err) => write!(f, "I/O error: {}", err),
            SicompilerError::TokenizationError(msg) => write!(f, "Tokenization error: {}", msg),
            SicompilerError::ValidationError(msg) => write!(f, "Validation error: {}", msg)
        }
    }
}

impl std::error::Error for SicompilerError {}

impl From<io::Error> for SicompilerError {
    fn from(value: io::Error) -> Self {
        SicompilerError::Io(value)
    }
}