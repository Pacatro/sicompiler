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
fn main() {
    let args: Cli = Cli::parse();
    
    let now: Instant = Instant::now();
    
    let repertorie_path: String;
    
    match args.repertorie_path {
        Some(path) => repertorie_path = path,
        None => repertorie_path = String::from("src/config/default-repertorie.txt"),
    }
    
    let tokenizer: Tokenizer = Tokenizer::new(args.input_path);
    let repertories: HashMap<String, Instruction>;

    match Tokenizer::tokenize_repertoire(&repertorie_path) {
        Ok(result) => repertories = result,
        Err(err) => {
            println!("Error: {}", err);
            return;
        } 
    }

    let tokens: Program;

    match tokenizer.tokenize() {
        Ok(result) => tokens = result,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    }

    let validator: Validator = Validator::new(tokens, args.output_path);
    
    match validator.validate(&repertories) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        },
    }

    println!("Finished in {}s", now.elapsed().as_secs_f32());
}
