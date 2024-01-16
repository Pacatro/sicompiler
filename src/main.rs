use clap::Parser;
use std::{time::Instant, process, error::Error};

use sicompiler::models::args::Cli;

fn main() {
    let cli: Cli = Cli::parse();

    let now: Instant = Instant::now();

    sicompiler::run(&cli).unwrap_or_else(|err: Box<dyn Error>| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    
    println!("Finished in {}s", now.elapsed().as_secs_f32());
}
