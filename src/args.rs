use clap::Parser;

#[derive(Parser)]
#[command(name = "SiCompailer", author, version, about, long_about = None)]
pub struct Cli {
    /// The input path to compile
    pub input_path: String,

    /// The output path to write to
    pub output_path: String,

    /// The repertoire of instructions
    #[arg(short = 'r', long = "repertoire")]
    pub repertoire_path: Option<String>,
}