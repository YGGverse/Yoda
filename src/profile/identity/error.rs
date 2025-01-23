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
            Self::Auth(e) => write!(f, "Could not create auth: {e}"),
            Self::Certificate(e) => {
                write!(f, "Could not create certificate: {e}")
            }
            Self::Database(e) => {
                write!(f, "Database error: {e}")
            }
            Self::Memory(e) => write!(f, "Memory error: {e}"),
        }
    }
}
