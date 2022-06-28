pub mod error;
pub mod opcode;
pub mod parser;

pub(crate) mod show;

use error::CliError;
use opcode::Opcode;
use parser::Program;

use std::collections::VecDeque;
use std::io::{self, ErrorKind, Read};
use std::slice;

#[derive(Default)]
pub struct Vm {
    tape: VecDeque<u8>,
    slots: Vec<usize>,
    calls: Vec<usize>,
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

    pub fn run(&mut self) -> Result<(), CliError> {
        while self.pc < self.code.len() {
            let opcode = self.fetch();
            self.execute(opcode)?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), CliError> {
        if self.pc < self.code.len() {
            let opcode = self.fetch();
            return self.execute(opcode);
        }

        Ok(())
    }

    fn execute(&mut self, opcode: Opcode) -> Result<(), CliError> {
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
                        Err(CliError::new(e))
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

                    self.pc += offset;
                }
            }
            Opcode::Until => {
                if self.tape[self.cell] != 0 {
                    let mut acc: usize = 0;
                    let pc = self.code[..self.pc]
                        .iter()
                        .rposition(|opcode| {
                            match opcode {
                                Opcode::While => acc -= 1,
                                Opcode::Until => acc += 1,
                                _ => (),
                            }

                            acc == 0
                        })
                        .unwrap();

                    self.pc = pc;
                }
            }

            Opcode::Def => {
                let i = self.tape[self.cell] as usize;

                self.slots.resize(usize::max(self.slots.len(), i + 1), 0);
                self.slots[i] = self.pc;

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

            Opcode::End | Opcode::Ret => self.pc = self.calls.pop().unwrap_or(usize::MAX),
            Opcode::Call => {
                let i = self.tape[self.cell] as usize;
                if let Some(pc) = self.slots.get(i).copied() {
                    self.calls.push(self.pc);
                    self.tape[self.cell] = 0;
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
