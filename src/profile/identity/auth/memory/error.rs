use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Clear,
    Overwrite(String),
    Unexpected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Clear => write!(f, "Could not cleanup memory index"),
            Self::Overwrite(key) => {
                write!(f, "Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => write!(f, "Unexpected error"),
        }
    }
}
