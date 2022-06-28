use crate::show::Show;
use std::error::Error;
use std::fmt;
use std::ops::Deref;

pub struct CliError(Box<dyn Show>);

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self(Box::new(message))
    }
}

impl From<&'static str> for CliError {
    fn from(message: &'static str) -> Self {
        Self(Box::new(message))
    }
}

impl CliError {
    pub fn new(error: impl 'static + Show) -> Self {
        Self(Box::new(error))
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

impl Error for CliError {}

#[derive(Default)]
pub struct CliErrors(Vec<CliError>);

impl<E> From<E> for CliErrors
where
    E: Into<CliError>,
{
    fn from(error: E) -> Self {
        Self(vec![error.into()])
    }
}

impl<E> FromIterator<E> for CliErrors
where
    E: Into<CliError>,
{
    fn from_iter<I: IntoIterator<Item = E>>(errors: I) -> Self {
        Self(errors.into_iter().map(Into::into).collect())
    }
}

impl CliErrors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, error: CliError) {
        self.0.push(error);
    }
}

impl Deref for CliErrors {
    type Target = [CliError];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for CliErrors {
    type Item = CliError;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for CliErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        self.0.iter().try_for_each(|e| writeln!(f, "  {e:?}"))
    }
}

impl fmt::Display for CliErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        self.0.iter().try_for_each(|e| writeln!(f, "  {e}"))
    }
}

impl Error for CliErrors {}
