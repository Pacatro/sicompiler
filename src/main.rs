mod tokenizer;
mod validator;
mod models;
mod args;

use args::Cli;
use tokenizer::Tokenizer;
use validator::Validator;
use models::{program::Program, instruction::Instruction};

use clap::Parser;
use std::{time::Instant, collections::HashMap};

// TODO: Restructure project
// TODO: Check how to know what number of arguments recive the instruction
fn main() {
    let args: Cli = Cli::parse();
    
    let now: Instant = Instant::now();
    
    let repertoire_path: String = match args.repertoire_path {
        Some(path) => path,
        None => String::from("src/config/default-repertoire.rep"),
    };
    
    let tokenizer: Tokenizer = Tokenizer::new(args.input_path);

    let repertoire: HashMap<String, Instruction> = match Tokenizer::tokenize_repertoire(&repertoire_path) {
        Ok(result) => result,
        Err(err) => {
            println!("Error: {}", err);
            return;
        } 
    };

    let tokens: Program = match tokenizer.tokenize() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let validator: Validator = Validator::new(tokens, args.output_path);
    
    match validator.validate(&repertoire) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        },
    }

    println!("Finished in {}s", now.elapsed().as_secs_f32());
}
