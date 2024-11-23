use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Memory(super::memory::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Database(reason) => write!(f, "Database error: {reason}"),
            Self::Memory(reason) => write!(f, "Memory error: {reason}"),
        }
    }
}
