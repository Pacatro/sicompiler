use clap::Parser;

#[derive(Parser)]
#[command(name = "SiCompailer", author, version, about, long_about = None)]
pub struct Cli {
    /// The input path to compile
    pub input_path: String,

    /// The output path to write to
    pub output_path: String,

    /// The repertorie of instructions
    #[arg(short = 'r', long = "repertorie")]
    pub repertorie_path: Option<String>,
}