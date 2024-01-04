mod tokenizer;

use tokenizer::Tokenizer;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Error: you should pass the input and output files path");
        println!("Example: sc <input_file> <output_file>");
        return;
    }

    let input_file: String = args[1].clone();
    // let output_file: String = args[2].clone();

    let tokenizer: Tokenizer = Tokenizer::new(input_file);

    match tokenizer.tokenize() {
        Ok(tokens) => println!("{:?}", tokens),
        Err(error) => eprintln!("Error: {error}")
    };

    //  TODO: Implements VALIDATOR struct.
}
