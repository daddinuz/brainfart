use crate::show::Show;

use std::{error, fmt};

pub struct CliError(Box<dyn Show>);

impl CliError {
    pub fn new(message: impl 'static + Show) -> Self {
        Self(Box::new(message))
    }
}

impl fmt::Debug for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for CliError {}

#[derive(Default)]
pub struct Errors(Vec<CliError>);

impl From<CliError> for Errors {
    fn from(error: CliError) -> Self {
        Self(vec![error])
    }
}

impl<T, const N: usize> From<[T; N]> for Errors
where
    T: 'static + Show,
{
    fn from(errors: [T; N]) -> Self {
        Self(errors.into_iter().map(CliError::new).collect())
    }
}

impl From<Vec<CliError>> for Errors {
    fn from(errors: Vec<CliError>) -> Self {
        Self(errors)
    }
}

impl Errors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, error: CliError) {
        self.0.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for Errors {
    type Item = CliError;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        self.0.iter().try_for_each(|e| writeln!(f, "  {e:?}"))
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        self.0.iter().try_for_each(|e| writeln!(f, "  {e}"))
    }
}

impl error::Error for Errors {}
