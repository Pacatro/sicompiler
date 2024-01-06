use super::{variable::Variable, init::Init, instruction::Instruction};

#[derive(Debug)]
pub struct Program(Vec<Variable>, Init, Vec<Instruction>);

impl Program {
    pub fn new(variables: Vec<Variable>, init: Init, instructions: Vec<Instruction>) -> Program {
        Program(variables, init, instructions)
    }

    pub fn variables(&self) -> &Vec<Variable> { &self.0 }
    pub fn init(&self) -> &Init { &self.1 }
    pub fn instructions(&self) -> &Vec<Instruction> { &self.2 }
}