use clap::Parser;

#[derive(Parser)]
#[command(name = "Sicompiler", author, version, about, long_about = None)]
pub struct Cli {
    /// The input path to compile
    pub input_path: String,

    #[arg(short = 'o', long = "out", default_value = "out.txt")]
    /// The output path to write to
    pub output_path: String,

    /// The repertoire of instructions
    #[arg(short = 'r', long = "rep")]
    pub repertoire_path: String,
}