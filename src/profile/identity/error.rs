use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Gemini(super::gemini::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Database(reason) => {
                write!(f, "Database error: {reason}")
            }
            Self::Gemini(reason) => {
                write!(f, "Could not init Gemini identity: {reason}")
            }
        }
    }
}
