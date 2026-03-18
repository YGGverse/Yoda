use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Parse,
    Syntect(syntect::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Parse => write!(f, "Parse error"),
            Self::Syntect(e) => {
                write!(f, "Syntect error: {e}")
            }
        }
    }
}
