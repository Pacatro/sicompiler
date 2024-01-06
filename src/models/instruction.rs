pub struct Instruction {
    mnemonic: String,
    flag: bool,
    params: Vec<String>,
}

impl Instruction {
    pub fn new(mnemonic: String, params: Vec<String>) -> Instruction {
        if params.len() == 0 {
            return Instruction { mnemonic, flag: false, params: vec![] };
        }
        
        Instruction { mnemonic, flag: true, params }
    }

    pub fn mnemonic(&self) -> &str { &self.mnemonic }
    pub fn flag(self) -> bool { self.flag }
    pub fn params(self) -> Vec<String> { self.params }
}