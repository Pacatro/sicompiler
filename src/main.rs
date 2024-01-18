use clap::Parser;
use std::{time::Instant, process};

use sicompiler::{models::args::Cli, errors::error::SicompilerError};

fn main() {
    let cli: Cli = Cli::parse();

    let now: Instant = Instant::now();

    sicompiler::run(&cli).unwrap_or_else(|err: SicompilerError| {
        eprintln!("{err}");
        process::exit(1);
    });
    
    println!("Finished in {}s", now.elapsed().as_secs_f32());
}
