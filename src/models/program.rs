use super::{variable::Variable, init::Init, instruction::Instruction};

/// Represents a complete program, consisting of variables, initialization information, and instructions.
#[derive(Debug)]
pub struct Program(Vec<Variable>, Init, Vec<Instruction>);

impl Program {
    /// Creates a new `Program` with the specified variables, initialization information, and instructions.
    ///
    /// ## Arguments
    ///
    /// - `variables` - The variables used in the program.
    /// - `init` - The initialization information for the program.
    /// - `instructions` - The instructions comprising the program.
    ///
    /// ## Returns
    ///
    /// A new `Program` instance.
    pub fn new(variables: Vec<Variable>, init: Init, instructions: Vec<Instruction>) -> Program {
        Program(variables, init, instructions)
    }

    pub fn variables(&self) -> &Vec<Variable> { &self.0 }
    pub fn init(&self) -> &Init { &self.1 }
    pub fn instructions(&self) -> &Vec<Instruction> { &self.2 }
}