pub mod models;
pub mod structs;
pub mod errors;

use std::collections::HashMap;

use structs::{tokenizer::Tokenizer, validator::Validator};
use models::{program::Program, instruction::Instruction, args::Cli};
use errors::error::SicompilerError;

/// Executes the main functionality of the program based on the provided command-line arguments.
///
/// ## Arguments
///
/// - `cli` - A reference to the `Cli` struct containing command-line arguments.
///
/// ## Returns
///
/// Returns a `Result` indicating success (`Ok(())`) or an error (`Err(SicompilerError)`).
///
pub fn run(cli: &Cli) -> Result<(), SicompilerError>{
    let tokenizer: Tokenizer = Tokenizer::new(&cli.input_path, &cli.repertoire_path);

    let repertoire: HashMap<String, Instruction> = tokenizer.tokenize_repertoire()?;

    let tokens: Program = tokenizer.tokenize()?;

    let validator: Validator = Validator::new(tokens, &cli.output_path);
    
    validator.validate(&repertoire)?;

    Ok(())
}