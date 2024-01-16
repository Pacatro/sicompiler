/// Represents an instruction in a program.
#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    mnemonic: String,
    flag: bool,
    params: Vec<String>,
}

impl Instruction {
    /// Creates a new `Instruction` with the specified mnemonic and parameters.
    ///
    /// ## Arguments
    ///
    /// - `mnemonic` - The mnemonic of the instruction.
    /// - `params` - A vector of string parameters associated with the instruction.
    ///
    /// ## Returns
    ///
    /// A new `Instruction` instance.
    pub fn new(mnemonic: &str, params: Vec<&str>) -> Instruction {
        if params.len() == 0 {
            return Instruction { mnemonic: String::from(mnemonic), flag: false, params: vec![] };
        }
        
        Instruction { mnemonic: String::from(mnemonic), flag: true, params: params.iter().map(|s| String::from(*s)).collect() }
    }

    pub fn mnemonic(&self) -> &str { &self.mnemonic }
    pub fn flag(&self) -> bool { self.flag }
    pub fn params(&self) -> &Vec<String> { &self.params }

    pub fn set_flag(&mut self, flag: bool) { self.flag = flag; }
    pub fn set_params(&mut self, params: Vec<&str>) { self.params = params.iter().map(|s| String::from(*s)).collect(); }
}