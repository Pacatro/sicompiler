mod tokenizer;
mod validator;
mod models;

use tokenizer::Tokenizer;
use validator::Validator;
use models::program::Program;

use std::{env, time::Instant};

// TODO: Implemets instruction repertoires (./sc <input_file> <output_file> -r <repertory.txt>)
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Error: you should pass the input and output files path");
        eprintln!("Usage: ./sc <input_file> <output_file>");
        return;
    }

    let input_file: String = args[1].clone();
    let output_file: String = args[2].clone();

    // let input_file: String = "examples/input.txt".to_string();
    // let output_file: String = "examples/out.txt".to_string();

    let now: Instant = Instant::now();

    let tokenizer: Tokenizer = Tokenizer::new(input_file);
    let tokens: Program;

    match tokenizer.tokenize() {
        Ok(result) => tokens = result,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    }

    let validator: Validator = Validator::new(tokens, output_file);
    
    match validator.validate() {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        },
    }

    println!("Finished in {}s", now.elapsed().as_secs_f32());
}
