use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::opcode::Opcode;
use crate::program::Program;

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

    pub fn parse(&mut self, stream: impl IntoIterator<Item = char>) -> Result<Program, ParseError> {
        let mut program = Program::new();

        for c in stream.into_iter() {
            match self.step(c) {
                Ok(Some(opcode)) => program.push(opcode),
                Err(error) => return Err(error),
                _ => continue,
            }
        }

        if self.brackets > 0 {
            return Err(ParseError::new(self.line, self.col, "unclosed `[`"));
        }

        if self.braces > 0 {
            return Err(ParseError::new(self.line, self.col, "unclosed `{`"));
        }

        Ok(program)
    }

    pub fn step(&mut self, c: char) -> Result<Option<Opcode>, ParseError> {
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
                    Ok(Some(Opcode::Until))
                } else {
                    Err(ParseError::new(self.line, self.col, "mismatching `]`"))
                }
            }

            '{' => {
                self.braces += 1;
                Ok(Some(Opcode::Def))
            }
            '}' => {
                if self.braces > 0 {
                    self.braces -= 1;
                    Ok(Some(Opcode::End))
                } else {
                    Err(ParseError::new(self.line, self.col, "mismatching `}`"))
                }
            }

            '\\' => Ok(Some(Opcode::Ret)),
            '@' => Ok(Some(Opcode::Call)),

            _ => Ok(None),
        };

        self.col += 1;
        result
    }
}

pub fn parse(stream: impl IntoIterator<Item = char>) -> Result<Program, ParseError> {
    let mut parser = Parser::new();
    parser.parse(stream)
}

#[derive(Debug)]
pub struct ParseError(Box<dyn Error>);

impl ParseError {
    fn new(line: usize, col: usize, message: &str) -> Self {
        Self(format!("At '{line}:{col}': {message}").into())
    }

    pub fn into_inner(self) -> Box<dyn Error> {
        self.0
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}
