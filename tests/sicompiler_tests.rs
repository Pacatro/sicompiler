use std::fs;

use sicompiler::{self, models::args::Cli, errors::error::SicompilerError};

const OUTPUT: &str = "0 0003
1 0003
3 0000
@
6
@
ADD 1
HALT 
";

#[test]
fn compile_success() -> Result<(), SicompilerError> {
    let cli: Cli = Cli {
        input_path: "tests-files/test-input.txt".to_string(),
        output_path: "tests-files/test-out.txt".to_string(),
        repertoire_path: "tests-files/test-repertoire.rep".to_string()
    };

    sicompiler::run(&cli)?;

    let result: String = fs::read_to_string("tests-files/test-out.txt")?;

    assert_eq!(result, OUTPUT);

    Ok(())
}

#[test]
fn compile_fails() {
    let cli: Cli = Cli {
        input_path: "tests-files/fails-files/bad-test-input.txt".to_string(),
        output_path: "tests-files/fails-files/bad-test-out.txt".to_string(),
        repertoire_path: "tests-files/test-repertoire.rep".to_string()
    };

    assert!(sicompiler::run(&cli).is_err());
}