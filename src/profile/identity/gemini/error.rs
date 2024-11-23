use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    Auth(super::auth::Error),
    Certificate(Box<dyn std::error::Error>),
    Database(sqlite::Error),
    Memory(super::memory::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Auth(reason) => write!(f, "Could not create auth: {reason}"),
            Self::Certificate(reason) => {
                write!(f, "Could not create certificate: {reason}")
            }
            Self::Database(reason) => {
                write!(f, "Database error: {reason}")
            }
            Self::Memory(reason) => write!(f, "Memory error: {reason}"),
        }
    }
}
