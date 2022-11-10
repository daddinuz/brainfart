pub mod opcode;
pub mod parser;
pub mod program;

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{self, ErrorKind, Read};
use std::{mem, slice};

use opcode::Opcode;
use program::Program;

// Special marker used to indicate that
// no jump must be performed, execution
// continues with the next instruction
const JUMP_NEXT: usize = usize::MAX;

#[derive(Default)]
pub struct Vm {
    tape: VecDeque<u8>,
    jump: Vec<usize>,
    link: Vec<usize>,
    code: Vec<Opcode>,
    cell: usize,
    pc: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(program: Program) -> Self {
        let tape = [0].into_iter().collect();
        let code = program.into_inner();

        Self {
            tape,
            code,
            ..Default::default()
        }
    }

    pub fn extend_program(&mut self, program: Program) {
        self.code.append(&mut program.into_inner());
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.pc < self.code.len() {
            let opcode = self.fetch();
            self.execute(opcode)?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        if self.pc < self.code.len() {
            let opcode = self.fetch();
            return self.execute(opcode);
        }

        Ok(())
    }

    fn execute(&mut self, opcode: Opcode) -> Result<(), RuntimeError> {
        match opcode {
            Opcode::Left => {
                if self.cell > 0 {
                    self.cell -= 1;
                } else {
                    self.tape.push_front(0);
                }
            }
            Opcode::Right => {
                self.cell += 1;
                if self.cell <= self.tape.len() {
                    self.tape.push_back(0);
                }
            }

            Opcode::Decr => self.tape[self.cell] -= 1,
            Opcode::Incr => self.tape[self.cell] += 1,

            Opcode::Read => io::stdin()
                .read_exact(slice::from_mut(&mut self.tape[self.cell]))
                .or_else(|e| {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        self.tape[self.cell] = 0;
                        Ok(())
                    } else {
                        Err(RuntimeError(e.into()))
                    }
                })?,
            Opcode::Write => print!("{}", self.tape[self.cell] as char),

            Opcode::While => {
                if self.tape[self.cell] == 0 {
                    let mut acc: usize = 1;
                    let offset = self.code[self.pc..]
                        .iter()
                        .position(|opcode| {
                            match opcode {
                                Opcode::While => acc += 1,
                                Opcode::Until => acc -= 1,
                                _ => (),
                            }

                            acc == 0
                        })
                        .unwrap();

                    self.pc += offset + 1;
                }
            }
            Opcode::Until => {
                if self.tape[self.cell] != 0 {
                    let mut acc: usize = 1;
                    let offset = self.code[..self.pc - 1]
                        .iter()
                        .rev()
                        .position(|opcode| {
                            match opcode {
                                Opcode::While => acc -= 1,
                                Opcode::Until => acc += 1,
                                _ => (),
                            }

                            acc == 0
                        })
                        .unwrap();

                    self.pc -= offset + 1;
                }
            }

            Opcode::Def => {
                let index = mem::replace(&mut self.tape[self.cell], 0) as usize;
                let new_len = usize::max(self.jump.len(), index + 1);

                self.jump.resize(new_len, JUMP_NEXT);
                self.jump[index] = self.pc;

                let mut acc: usize = 1;
                let offset = self.code[self.pc..]
                    .iter()
                    .position(|opcode| {
                        match opcode {
                            Opcode::Def => acc += 1,
                            Opcode::End => acc -= 1,
                            _ => (),
                        }

                        acc == 0
                    })
                    .unwrap();

                self.pc += offset + 1;
            }

            Opcode::End | Opcode::Ret => self.pc = self.link.pop().unwrap_or(usize::MAX),
            Opcode::Call => {
                // current cell must be set to zero in any case
                let index = mem::replace(&mut self.tape[self.cell], 0) as usize;

                if let Some(pc) = self.jump.get(index).copied().filter(|pc| *pc != JUMP_NEXT) {
                    self.link.push(self.pc);
                    self.pc = pc;
                }
            }
        }

        Ok(())
    }

    fn fetch(&mut self) -> Opcode {
        let opcode = self.code[self.pc];
        self.pc += 1;
        opcode
    }
}

#[derive(Debug)]
pub struct RuntimeError(Box<dyn Error>);

impl RuntimeError {
    pub fn into_inner(self) -> Box<dyn Error> {
        self.0
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for RuntimeError {}
