use std::str::FromStr;

use crate::opcode::Opcode;
use crate::parser::{self, ParseError};

#[derive(Debug)]
pub struct Program(Vec<Opcode>);

impl Program {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn push(&mut self, opcode: Opcode) {
        self.0.push(opcode);
    }

    pub(crate) fn into_inner(self) -> Vec<Opcode> {
        self.0
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        parser::parse(program.chars())
    }
}
