mod error;

use std::{env, fs, io};

use brainfart::Vm;

use crate::error::CliError;

fn main() -> Result<(), CliError> {
    let exe = env!("CARGO_PKG_NAME");
    let args: Vec<String> = env::args().skip(1).collect();

    let (interactive, file) = match &args[..] {
        [mode, file] if mode == "-i" => (true, Some(file)),
        [mode] if mode == "-i" => (true, None),
        [file] => (false, Some(file)),
        [] => (true, None),
        _ => return Err(CliError::from(format!("{exe} [-i] [file]"))),
    };

    let source = file.map_or_else(|| Ok(String::new()), fs::read_to_string)?;
    let program = source.parse()?;
    let mut vm = Vm::load(program);

    if !interactive {
        vm.run()?;
        return Ok(());
    }

    if let Err(error) = vm.run() {
        eprintln!("Error: {error}");
    }

    while let Err(error) = repl(&mut vm) {
        eprintln!("Error: {error}");
    }

    Ok(())
}

fn repl(vm: &mut Vm) -> Result<(), CliError> {
    let stdin = io::stdin();
    let mut line = String::new();

    while stdin.read_line(&mut line)? > 0 {
        let program = line.parse()?;

        vm.extend_program(program);
        vm.run()?;

        line.clear();
    }

    Ok(())
}
