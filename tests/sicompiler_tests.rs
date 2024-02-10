use std::fs;

use sicompiler::{self, models::args::Cli, errors::error::SicompilerError};

const OUTPUT: &str = "0 0003
1 0003
3 0000
@
6
@
CRA 
CRF 
ADD 0001
ROR_F_ACC 
SFZ 
JMP 12
CRA 
CRF 
ADD 0000
ROL_F_ACC 
STA 0004
HALT 
CRA 
CRF 
ADD 0001
ROL_F_ACC 
STA 0004
HALT 
";

#[test]
fn compile_success() -> Result<(), SicompilerError> {
    let cli: Cli = Cli {
        input_path: "tests/utils/test-input.txt".to_string(),
        output_path: "tests/utils/test-out.txt".to_string(),
        repertoire_path: "tests/utils/test-repertoire.rep".to_string()
    };

    sicompiler::run(&cli)?;

    let result: String = fs::read_to_string("tests/utils/test-out.txt")?;

    assert_eq!(result, OUTPUT);

    Ok(())
}

#[test]
fn compile_fails() {
    let cli: Cli = Cli {
        input_path: "tests/utils/bad-test-input.txt".to_string(),
        output_path: "tests/utils/bad-test-out.txt".to_string(),
        repertoire_path: "tests/utils/test-repertoire.rep".to_string()
    };

    assert!(sicompiler::run(&cli).is_err());
}