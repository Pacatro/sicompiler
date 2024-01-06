mod tokenizer;
mod validator;
mod models;

use tokenizer::Tokenizer;
use validator::Validator;
use models::program::Program;

use std::{env, time::Instant, io::Error, io::ErrorKind};

// TODO: IMPLEMENT CUSTOM ERRORS
// TODO: CHECK STANDARS FOR THE PROGRAM STRUCT
// TODO: CONTEMPLATE ALL ERROR CASES

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(Error::new(ErrorKind::Other, "Error: you should pass the input and output files path\nExample: sc <input_file> <output_file>"));
    }

    let input_file: String = args[1].clone();
    let output_file: String = args[2].clone();

    // let input_file: String = "examples/input.txt".to_string();
    // let output_file: String = "examples/out.txt".to_string();

    let now: Instant = Instant::now();

    let tokenizer: Tokenizer = Tokenizer::new(input_file);
    let tokens: Program = tokenizer.tokenize()?;

    let validator: Validator = Validator::new(tokens, output_file);
    validator.validate()?;

    println!("Finished in {}s", now.elapsed().as_secs_f32());

    Ok(())
}
