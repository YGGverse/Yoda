#[derive(Debug)]
pub enum Error {
    Auth(super::auth::Error),
    Certificate(Box<dyn std::error::Error>),
    Database(sqlite::Error),
    Memory(super::memory::Error),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Auth(reason) => format!("Could not create auth: {}", reason.to_string()),
            Self::Certificate(reason) => {
                format!("Could not create certificate: {}", reason.to_string())
            }
            Self::Database(reason) => {
                format!("Database error: {}", reason.to_string())
            }
            Self::Memory(reason) => format!("Memory error: {}", reason.to_string()),
        }
    }
}
