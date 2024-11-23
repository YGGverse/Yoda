use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Overwrite(String),
    Unexpected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Overwrite(key) => {
                write!(f, "Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => write!(f, "Unexpected error"),
        }
    }
}
