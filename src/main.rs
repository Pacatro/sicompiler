mod tokenizer;
mod validator;

pub mod models;

use tokenizer::Tokenizer;
use validator::Validator;
use models::{instruction::Instruction, variable::Variable, init::Init};

use std::{env, time::Instant};


fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 3 {
    //     eprintln!("Error: you should pass the input and output files path");
    //     eprintln!("Example: sc <input_file> <output_file>");
    //     return;
    // }

    // let input_file: String = args[1].clone();
    // let output_file: String = args[2].clone();

    let input_file: String = "examples/input.txt".to_string();
    let output_file: String = "examples/out.txt".to_string();

    let now: Instant = Instant::now();

    let tokenizer: Tokenizer = Tokenizer::new(input_file);

    let tokens: (Vec<Variable>, Init, Vec<Instruction>);

    match tokenizer.tokenize() {
        Ok(result) => tokens = result,
        Err(error) => eprintln!("Error: {error}")
    };

    println!("Finished in {}s", now.elapsed().as_secs_f32());

    // TODO: Implements program structure validation

    // let validator: Validator = Validator::new(tokens, output_file);

    // match validator.validate() {
    //     Ok(_) => println!("Finished in {}s", now.elapsed().as_secs_f32()),
    //     Err(error) => eprintln!("Error: {error}")
    // };
}
