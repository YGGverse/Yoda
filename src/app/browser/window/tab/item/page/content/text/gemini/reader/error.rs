use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Gemtext(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Gemtext(e) => {
                write!(f, "Gemtext error: {e}")
            }
        }
    }
}
