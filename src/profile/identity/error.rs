#[derive(Debug)]
pub enum Error {
    Database(sqlite::Error),
    Gemini(super::gemini::Error),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Database(reason) => {
                format!("Database error: {}", reason.to_string())
            }
            Self::Gemini(reason) => {
                format!("Could not init Gemini identity: {}", reason.to_string())
            }
        }
    }
}
