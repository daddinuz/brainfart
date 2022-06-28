use brainfart::error::{CliError, CliErrors};
use brainfart::Vm;

use std::{env, fs, io};

fn main() -> Result<(), CliErrors> {
    let exe = env!("CARGO_PKG_NAME");
    let args: Vec<String> = env::args().skip(1).collect();

    let (interactive, file) = match &args[..] {
        [mode, file] if mode == "-i" => (true, Some(file)),
        [mode] if mode == "-i" => (true, None),
        [file] => (false, Some(file)),
        [] => (true, None),
        _ => return Err(CliErrors::from(format!("{exe} [-i] [file]"))),
    };

    let program = file
        .map_or_else(|| Ok(String::new()), fs::read_to_string)
        .map_err(CliError::new)?;

    let mut vm = Vm::load(program.parse()?);
    vm.run()?;

    if interactive {
        return repl(&mut vm);
    }

    Ok(())
}

fn repl(vm: &mut Vm) -> Result<(), CliErrors> {
    let stdin = io::stdin();
    let mut line = String::new();

    while stdin.read_line(&mut line).map_err(CliError::new)? > 0 {
        match line.parse() {
            Ok(program) => {
                vm.extend_program(program);
                vm.run()?;
            }
            Err(errors) => errors.into_iter().for_each(|e| eprintln!("{e}")),
        }

        line.clear();
    }

    Ok(())
}
