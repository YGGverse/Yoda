#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Memory(super::memory::Error),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Database(reason) => format!("Database error: {}", reason),
            Self::Memory(reason) => format!("Memory error: {}", reason.to_string()),
        }
    }
}
