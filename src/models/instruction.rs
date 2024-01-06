#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    mnemonic: String,
    flag: bool,
    params: Vec<String>,
}

impl Instruction {
    pub fn new(mnemonic: &str, params: Vec<&str>) -> Instruction {
        if params.len() == 0 {
            return Instruction { mnemonic: String::from(mnemonic), flag: false, params: vec![] };
        }
        
        Instruction { mnemonic: String::from(mnemonic), flag: true, params: params.iter().map(|s| String::from(*s)).collect() }
    }

    pub fn mnemonic(&self) -> &str { &self.mnemonic }
    pub fn flag(self) -> bool { self.flag }
    pub fn params(self) -> Vec<String> { self.params }
}