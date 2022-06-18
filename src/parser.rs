use crate::error::{CliError, Errors};
use crate::opcode::Opcode;

use std::str::FromStr;

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
    type Err = Errors;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        parse(program.chars())
    }
}

#[derive(Default)]
pub struct Parser {
    brackets: usize,
    braces: usize,
    line: usize,
    col: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, stream: impl IntoIterator<Item = char>) -> Result<Program, Errors> {
        let mut program = Program::new();
        let mut errors = Errors::new();

        for c in stream.into_iter() {
            match self.step(c) {
                Ok(Some(opcode)) if errors.is_empty() => program.push(opcode),
                Err(error) => errors.push(error),
                _ => (),
            }
        }

        if self.brackets > 0 {
            errors.push(CliError::new(format!(
                "At {}:{}: unclosed `[`",
                self.line, self.col
            )));
        }

        if self.braces > 0 {
            errors.push(CliError::new(format!(
                "At {}:{}: unclosed `{{`",
                self.line, self.col
            )));
        }

        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }

    pub fn step(&mut self, c: char) -> Result<Option<Opcode>, CliError> {
        let result = match c {
            '\n' => {
                self.line += 1;
                self.col = 0;
                Ok(None)
            }

            '<' => Ok(Some(Opcode::Left)),
            '>' => Ok(Some(Opcode::Right)),

            '-' => Ok(Some(Opcode::Decr)),
            '+' => Ok(Some(Opcode::Incr)),

            ',' => Ok(Some(Opcode::Read)),
            '.' => Ok(Some(Opcode::Write)),

            '[' => {
                self.brackets += 1;
                Ok(Some(Opcode::While))
            }
            ']' => {
                if self.brackets > 0 {
                    self.brackets -= 1;
                    Ok(Some(Opcode::Repeat))
                } else {
                    Err(CliError::new(format!(
                        "At {}:{}: mismatching `]`",
                        self.line, self.col
                    )))
                }
            }

            '{' => {
                self.braces += 1;
                Ok(Some(Opcode::Bind))
            }
            '@' => Ok(Some(Opcode::Call)),
            '}' => {
                if self.braces > 0 {
                    self.braces -= 1;
                    Ok(Some(Opcode::Ret))
                } else {
                    Err(CliError::new(format!(
                        "At {}:{}: mismatching `}}`",
                        self.line, self.col
                    )))
                }
            }

            _ => Ok(None),
        };

        self.col += 1;
        result
    }
}

pub fn parse(stream: impl IntoIterator<Item = char>) -> Result<Program, Errors> {
    let mut parser = Parser::new();
    parser.parse(stream)
}
