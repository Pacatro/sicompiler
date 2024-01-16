pub mod models;
mod structs;

use std::{collections::HashMap, error::Error};

use structs::{tokenizer::Tokenizer, validator::Validator};
use models::{program::Program, instruction::Instruction, args::Cli};

/// Executes the main functionality of the program based on the provided command-line arguments.
///
/// ## Arguments
///
/// - `cli` - A reference to the `Cli` struct containing command-line arguments.
///
/// ## Returns
///
/// Returns a `Result` indicating success (`Ok(())`) or an error (`Err(Box<dyn Error>)`).
///
pub fn run(cli: &Cli) -> Result<(), Box<dyn Error>>{
    let tokenizer: Tokenizer = Tokenizer::new(cli.input_path.clone(), cli.repertoire_path.clone());

    let repertoire: HashMap<String, Instruction> = tokenizer.tokenize_repertoire()?;

    let tokens: Program = tokenizer.tokenize()?;

    let validator: Validator = Validator::new(tokens, cli.output_path.clone());
    
    validator.validate(&repertoire)?;

    Ok(())
}