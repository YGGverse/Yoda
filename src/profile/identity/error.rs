use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Gemini(super::gemini::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Database(e) => {
                write!(f, "Database error: {e}")
            }
            Self::Gemini(e) => {
                write!(f, "Could not init Gemini identity: {e}")
            }
        }
    }
}
