use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

use brainfart::parser::ParseError;
use brainfart::RuntimeError;

pub struct CliError(Box<dyn Error>);

impl From<RuntimeError> for CliError {
    fn from(error: RuntimeError) -> Self {
        Self(error.into_inner())
    }
}

impl From<ParseError> for CliError {
    fn from(error: ParseError) -> Self {
        Self(error.into_inner())
    }
}

impl From<IoError> for CliError {
    fn from(error: IoError) -> Self {
        Self(error.into())
    }
}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self(message.into())
    }
}

impl Debug for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for CliError {}
